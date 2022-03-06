use std::{env, fs, io};

fn main() -> io::Result<()> {
    let profile = env::var("PROFILE").unwrap_or("debug".to_string());
    let makefile = format!(
        r#"install:
	@mkdir -p ./lua/deps
	@rm -f ./lua/compleet.so
	@rm -f ./lua/deps/* || true
	@cp ./target/{profile}/libcompleet.so ./lua/compleet.so
	@cp ./target/{profile}/deps/*.rlib ./lua/deps
"#
    );
    fs::write("Makefile", &makefile)?;
    Ok(())
}
