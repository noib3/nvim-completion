mod common;

use common::nvim_execute;

#[test]
fn ping_pong() {
    let response = nvim_execute(&[
        r#"lua require('compleet').setup()"#,
        r#"lua require('compleet/testing').ping()"#,
    ]);

    assert_eq!(response, "Rust says pong!");
}
