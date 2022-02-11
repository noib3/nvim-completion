use std::{process::Command, str};

pub fn nvim_execute(cmds: &[&str]) -> String {
    let args = cmds
        .iter()
        .map(|&cmd| ["-c", cmd])
        .flatten()
        .collect::<Vec<&str>>();

    let raw_output = Command::new("nvim")
        .args(&["-u", "NONE", "--headless"])
        .args(&args)
        .args(&["+quit"])
        .output()
        .expect("Failed to execute command")
        .stderr;

    String::from_utf8(raw_output).expect("Found invalid UTF8")
}
