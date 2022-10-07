//! This module contains everything related to the initial setup of the plugin.
//!
//! This includes things like registering sources, building the plugin's Lua
//! API exposed to Neovim, and implementing the [`setup`] function responsible
//! for starting the plugin at runtime.

use std::cell::RefCell;
use std::collections::HashMap;

use completion_types::{
    CompletionSource, CoreSender, SourceBundle, SourceEnable, SourceId,
};
use nvim_oxi::{self as nvim, libuv::AsyncHandle, Object};
use once_cell::unsync::Lazy;
use tokio::sync::mpsc;

use crate::autocmds;
use crate::config::{Config, SourceConfig, SourcesConfig};
use crate::hlgroups;
use crate::messages::echoerr;
use crate::{Client, Error, Result, SourceBundleExt};

/// `Lazy` is to have a heap-allocated global variable, `RefCell` is for
/// interior mutability and `Option` allows to extract an owned `T` via
/// `Option::take`.
type StaticVar<T> = Lazy<RefCell<Option<T>>>;

thread_local! {
    static SOURCES: StaticVar<HashMap<SourceId, SourceBundle>> =
        Lazy::new(|| RefCell::new(Some(HashMap::new())));
}

// Temporarily stores the completion sources in a thread-local global
// variable.
//
// When the user calls the `require("nvim-completion").setup({..})` function
// we take the sources out of the global variable, filter out all the ones
// with `enable = false` and pass the resulting [`SourceMap`] to the spawned
// thread pool.
//
/// Has to be called in the Neovim thread.
pub fn register_source<S: CompletionSource>(source: S) {
    SOURCES.with(move |s| {
        let sources = &mut *s.borrow_mut();
        let sources = sources.as_mut().unwrap();
        sources.insert(S::NAME, SourceBundle::from(source));
    });
}

// /// TODO: docs
// pub(crate) fn register_runtime_source(path: String) -> nvim::Result<()> {
//     let (name, bundle) = unsafe {
//         let lib = Library::new(path).unwrap();

//         let (name, source) = lib
//             .get::<SourcePtr>(RUNTIME_SOURCE_EXPORTED_SYMBOL.as_bytes())
//             .unwrap()();

//         (name, SourceBundle::from_ptr(source))
//     };

//     SOURCES.with(move |s| {
//         let sources = &mut *s.borrow_mut();
//         let sources = sources.as_mut().unwrap();
//         sources.insert(name, bundle);
//     });

//     Ok(())
// }

// Returns the whole user-facing API of the plugin. The returned
// [`Dictionary`] is the Lua table users see when they inspect the plugin via:
//
// ``` lua
// local completion = require("nvim-completion")
// print(vim.inspect(completion))
// ```
//
// inside Neovim.
pub fn build_api() -> nvim::Dictionary {
    let client = Client::default();

    let source_apis = SOURCES.with(|s| {
        let sources = &*s.borrow();
        let sources = sources.as_ref().unwrap();

        sources
            .iter()
            .map(|(&name, bundle)| (name, bundle.source.api()))
            .collect::<Vec<_>>()
    });

    [
        // ("register_source", Function::from_fn(register_runtime_source).into()),
        ("setup", client.to_nvim_fn(self::setup).into()),
    ]
    .into_iter()
    .chain(crate::mappings::setup(&client))
    .chain(source_apis)
    .collect()
}

/// TODO: docs
fn setup(client: &Client, preferences: Object) -> Result<()> {
    if SOURCES.with(|s| s.borrow().is_none()) {
        return Err(Error::AlreadySetup);
    }

    // Set the highlight groups *before* deserializing the preferences so
    // that error messages will be displayed with the right colors.
    hlgroups::setup()?;

    let Config { sources, completion, ui } = Config::try_from(preferences)?;

    // Filter out all the non-enabled registered sources based on the sources
    // config.
    let sources = {
        let configs = sources;
        let mut sources = SOURCES.with(|s| s.borrow_mut().take().unwrap());
        self::filter_enabled(&mut sources, configs)?;
        sources.into_iter().map(|(_id, bundle)| bundle).collect::<Vec<_>>()
    };

    let augroup_id = autocmds::setup(client)?;

    let core_sender = self::register_main_callback(client.clone())?;

    let (client_sender, client_receiver) = mpsc::unbounded_channel();
    completion_core::start(sources, core_sender.clone(), client_receiver);

    client.init(augroup_id, core_sender, client_sender, completion, ui)?;

    Ok(())
}

/// TODO: docs
fn filter_enabled(
    sources: &mut HashMap<SourceId, SourceBundle>,
    configs: SourcesConfig,
) -> Result<()> {
    for (name, SourceConfig { enable, rest }) in configs {
        match enable {
            SourceEnable::Bool(false) => continue,

            enable => {
                let bundle = sources.get_mut(&*name).unwrap();
                bundle.set_config(rest)?;
                bundle.set_enable(enable);
            },
        }
    }

    sources.retain(|_, bundle| bundle.is_initialized());

    Ok(())
}

/// TODO: docs
fn register_main_callback(client: Client) -> Result<CoreSender> {
    let (sender, mut receiver) = mpsc::unbounded_channel();

    let handle = AsyncHandle::new(move || {
        if let Err(error) = client.handle_core_message(&mut receiver) {
            match error {
                Error::Nvim(e) => return Err(e),

                err if err.is_fatal() => nvim::schedule(move |_| {
                    echoerr!("FATAL: {}", err);
                    Ok(())
                }),

                other => nvim::schedule(move |_| Ok(echoerr!("{}", other))),
            }
        }
        Ok(())
    })?;

    Ok(CoreSender::new(sender, handle))
}

/// Returns a vector containing the names of the completion sources that have
/// been registered so far.
///
/// NOTE: this is used in the deserialization of the user config passed to
/// [`setup`]. Calling it after the config has been successfully deserialized
/// and the sources have been passed to the core will result in a panic.
pub(crate) fn registered_source_names() -> Vec<&'static str> {
    SOURCES.with(|sources| {
        sources.borrow().as_ref().unwrap().keys().map(|k| *k).collect()
    })
}
