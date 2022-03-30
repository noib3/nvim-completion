use std::sync::Arc;

use mlua::prelude::{Lua, LuaError, LuaResult, LuaValue};
use neovim::Neovim;
use parking_lot::Mutex;
use tokio::runtime;

use crate::completion::Completions;
use crate::settings::Settings;
use crate::state::State;
use crate::{autocmds, commands, hlgroups, mappings};

/// Executed by the `require("compleet").setup` Lua function.
pub fn setup(
    lua: &Lua,
    state: &Arc<Mutex<State>>,
    preferences: LuaValue,
) -> LuaResult<()> {
    let api = Neovim::new(lua)?.api;

    // Here we create the highlight groups used in the error messages.
    hlgroups::setup_error_msg(lua, &api)?;

    // If the Neovim version isn't >= 0.7 we echo an error message and return
    // early.
    if !api.call_function::<_, u8>("has", vec!["nvim-0.7"])? == 1 {
        let chunks = [
            ("[nvim-compleet]", Some("CompleetErrorMsgTag")),
            (" Neovim v0.7+ is required", None),
        ];
        api.echo(&chunks, true)?;
        return Ok(());
    }

    let _state = state.clone();
    let _state = &mut _state.lock();

    _state.settings = match preferences {
        LuaValue::Table(t) => {
            // Using the `serde_path_to_error` crate to get the full path
            // of the option where the error occured.
            match serde_path_to_error::deserialize::<_, Settings>(
                mlua::serde::Deserializer::new(LuaValue::Table(t)),
            ) {
                Ok(settings) => settings,

                Err(e) => match e.inner() {
                    LuaError::DeserializeError(msg) => {
                        let path = e.path().to_string();
                        let mut chunks = vec![
                            ("[nvim-compleet]", Some("CompleetErrorMsgTag")),
                            (" Error for `", None),
                            (&path, Some("CompleetErrorMsgOptionPath")),
                            ("`: ", None),
                        ];

                        chunks.append(&mut to_chunks(msg));

                        api.echo(&chunks, true)?;

                        return Ok(());
                    },

                    _ => return Err(e.into_inner()),
                },
            }
        },

        LuaValue::Nil => Settings::default(),

        _ => {
            let chunks = [
                ("[nvim-compleet]", Some("CompleetErrorMsgTag")),
                (" Invalid value '", None),
                (&format!("{:?}", preferences), Some("Statement")),
                ("'. Please pass either a table or ", None),
                ("nil", Some("Statement")),
                (" to the setup function", None),
            ];
            api.echo(&chunks, true)?;
            return Ok(());
        },
    };

    // TODO: print warning message and return.
    if _state.settings.sources.is_empty() {
        todo!()
    }

    #[cfg(debug)]
    {
        let nvim = Neovim::new(lua)?;
        nvim.print(format!("{:#?}", &_state.settings))?;
    }

    // Only execute this block the first time this function is called.
    if !_state.did_setup {
        let aux = autocmds::setup(lua, &api, state)?;
        _state.augroup_id = Some(aux.0);
        _state.try_buf_attach = Some(aux.1);

        commands::setup(lua, &api, state)?;
        hlgroups::setup(lua, &api)?;
        mappings::setup(lua, &api, state)?;

        // Create the multi-threaded async runtime with one thread per
        // completion source.
        _state.runtime = Some(
            runtime::Builder::new_multi_thread()
                .worker_threads(_state.settings.sources.len())
                .enable_io()
                // TODO: remove this, it's only for testing
                .enable_time()
                .thread_name("compleet-sources-pool")
                .build()
                .expect("Creating Tokio runtime"),
        );

        let (tx, rx) = tokio::sync::mpsc::channel::<Completions>(100);
        // let (tx, rx) = std::sync::mpsc::channel::<Completions>();
        _state.tx = Some(Arc::new(tx));
        _state.rx = Some(rx);

        _state.did_setup = true;
    }

    #[cfg(debug)]
    {
        nvim.print(format!(
            "State cloned {} times in total!",
            Arc::<Mutex<State>>::strong_count(state)
        ))?;
    }

    Ok(())
}

fn to_chunks(msg: &str) -> Vec<(&'_ str, Option<&'static str>)> {
    msg.split('`')
        .enumerate()
        .map(|(i, str)| match i % 2 == 1 {
            true => vec![
                ("`", None),
                (str, Some("CompleetErrorMsgField")),
                ("`", None),
            ],
            false => vec![(str, None)],
        })
        .flatten()
        .collect()
}
