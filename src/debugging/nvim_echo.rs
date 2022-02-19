use rmpv::Value;

use crate::Nvim;

/// Echoes a message to the Neovim client.
/// Basically just a more ergonomic wrapper around `nvim_rs::Neovim::echo`.
///
/// # Arguments
///
///   - `nvim`: a reference to the Neovim client;
///   - `msg`: a string slice representing the message to be shown;
///   - `hl_group`: the name of the highlight group to highlight the message
///   in;
///   - `add_to_history`: if true the message will be added to the message
///   history.
/// ```
pub async fn nvim_echo(
    nvim: &Nvim,
    msg: &str,
    hl_group: &str,
    add_to_history: bool,
) {
    let chunk = vec![Value::from(msg), Value::from(hl_group)];
    nvim.echo(vec![Value::from(chunk)], add_to_history, vec![])
        .await
        .unwrap();
}
