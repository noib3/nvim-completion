mod common;

use common::nvim_execute;

#[test]
fn ping_pong() {
    let response = nvim_execute(&[
        r#"lua require('compleet').setup()"#,
        r#"lua print(require('compleet').request('ping', 'Neovim says ping!'))"#,
    ]);
    assert_eq!(response, "Rust says pong!");
}
