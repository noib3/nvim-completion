//! This module contains everything related to the construction and
//! initialization of the final plugin.

use std::cell::RefCell;

use nvim::{r#loop::AsyncHandle, Object};
use nvim_oxi as nvim;
use once_cell::unsync::Lazy;
use tokio::sync::mpsc;

use crate::config::Config;
use crate::messages::echoerr;
use crate::pipeline::{MainSender, PoolSender};
use crate::sources::{
    CompletionSource,
    SourceBundle,
    SourceConfig,
    SourceConfigs,
    SourceEnable,
    SourceMap,
};
use crate::{Client, Error, Result};

/// `Lazy` is to have a heap-allocated global variable, `RefCell` is for
/// interior mutability and `Option` allows to extract an owned `T` via
/// `Option::take`.
type StaticVar<T> = Lazy<RefCell<Option<T>>>;

thread_local! {
    static SOURCES: StaticVar<SourceMap> =
        Lazy::new(|| RefCell::new(Some(SourceMap::new())));
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

    let source_dicts = SOURCES.with(|s| {
        let sources = &*s.borrow();
        let sources = sources.as_ref().unwrap();

        sources
            .iter()
            .map(|(&name, bundle)| (name, bundle.api()))
            .collect::<Vec<_>>()
    });

    [("setup", Object::from(client.to_nvim_fn(self::setup)))]
        .into_iter()
        .chain(crate::mappings::setup(&client))
        .chain(source_dicts)
        .collect()
}

/// TODO: docs
fn setup(client: &Client, preferences: Object) -> Result<()> {
    if SOURCES.with(|s| s.borrow().is_none()) {
        return Err(Error::AlreadySetup);
    }

    // Set the highlight groups *before* deserializing the preferences so
    // that error messages will be displayed with the right colors.
    crate::hlgroups::setup()?;

    let Config { sources, completion, ui } = Config::try_from(preferences)?;

    // Filter out all the non-enabled registered sources based on the sources
    // config.
    let sources = {
        let configs = sources;
        let mut sources = SOURCES.with(|s| s.borrow_mut().take().unwrap());
        self::filter_enabled(&mut sources, configs)?;
        sources
    };

    // Update the completion state with the completion config.
    {
        let state = &mut *client.completion();
        state.set_config(completion);
    }

    // Update the ui state with the ui config.
    {
        let state = &mut *client.ui();
        state.set_config(ui);
    }

    let augroup_id = self::setup_augroup(client)?;

    let (main_sender, handle) = self::register_main_callback(client.clone())?;

    let pool_sender =
        self::start_sources_pool(sources, main_sender.clone(), handle);

    client.set_augroup_id(augroup_id);
    client.set_main_sender(main_sender);
    client.set_pool_sender(pool_sender);

    Ok(())
}

/// TODO: docs
fn filter_enabled(
    sources: &mut SourceMap,
    configs: SourceConfigs,
) -> Result<()> {
    for (name, SourceConfig { enable, rest }) in configs {
        match enable {
            SourceEnable::Final(false) => continue,

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
fn setup_augroup(client: &Client) -> Result<u32> {
    const AUGROUP_NAME: &str = "nvim-completion";

    let opts = nvim::opts::CreateAugroupOpts::builder().clear(true).build();
    let augroup_id = nvim::api::create_augroup(AUGROUP_NAME, Some(&opts))?;
    crate::autocmds::setup(client, augroup_id)?;

    Ok(augroup_id)
}

/// TODO: docs
fn register_main_callback(
    client: Client,
) -> Result<(MainSender, AsyncHandle)> {
    let (sender, mut receiver) = mpsc::unbounded_channel();

    let handle = nvim::r#loop::new_async(move || {
        match crate::pipeline::main_cb(&client, &mut receiver) {
            Err(Error::Nvim(e)) => return Err(e),

            Err(other) => echoerr!("{}", other),

            Ok(_) => {},
        }

        Ok(())
    })?;

    Ok((sender, handle))
}

/// TODO: docs
fn start_sources_pool(
    sources: SourceMap,
    main_sender: MainSender,
    handle: AsyncHandle,
) -> PoolSender {
    let (sender, receiver) = mpsc::unbounded_channel();

    let _ = std::thread::spawn(move || {
        crate::pipeline::sources_pool(sources, receiver, main_sender, handle)
    });

    sender
}

/// TODO: docs
pub(crate) fn registered_source_names() -> Vec<&'static str> {
    SOURCES.with(|sources| {
        sources.borrow().as_ref().unwrap().keys().map(|k| *k).collect()
    })
}
