use nvim_oxi::{self as nvim, api, opts::SetKeymapOpts, types::Mode};

use crate::Client;

pub(crate) enum IdentifyCompletion {
    /// A specific completion.
    ByIndex(usize),

    /// A positive or negative offset from the currently selected completion.
    FromSelected(isize),
}

pub(crate) fn setup(client: &Client) -> nvim::Result<()> {
    let accept_first = client.create_fn(|client, _| {
        super::accept_completion(client, IdentifyCompletion::ByIndex(0))
    });

    let accept_selected = client.create_fn(|client, _| {
        super::accept_completion(client, IdentifyCompletion::FromSelected(0))
    });

    let _scroll_details = client.create_fn(super::scroll_details);

    let select_next = client.create_fn(|client, _| {
        super::select_completion(client, IdentifyCompletion::FromSelected(1))
    });

    let select_prev = client.create_fn(|client, _| {
        super::select_completion(client, IdentifyCompletion::FromSelected(-1))
    });

    let show = client.create_fn(|client, _| super::show_completions(client));

    let mut opts = SetKeymapOpts::builder();
    opts.silent(true);

    api::set_keymap(
        Mode::Insert,
        "<Plug>(compleet-accept-first)",
        "",
        Some(&opts.callback(accept_first).build()),
    )?;

    api::set_keymap(
        Mode::Insert,
        "<Plug>(compleet-accept-selected)",
        "",
        Some(&opts.callback(accept_selected).build()),
    )?;

    api::set_keymap(
        Mode::Insert,
        "<Plug>(compleet-select-next)",
        "",
        Some(&opts.callback(select_next).build()),
    )?;

    api::set_keymap(
        Mode::Insert,
        "<Plug>(compleet-select-prev)",
        "",
        Some(&opts.callback(select_prev).build()),
    )?;

    api::set_keymap(
        Mode::Insert,
        "<Plug>(compleet-show-completion)",
        "",
        Some(&opts.callback(show).build()),
    )?;

    Ok(())
}
