use std::{
    fs,
    io::{self, BufRead},
    path::Path,
    process::Command,
};

const SUPPORTED_FILETYPES: &[&str] = &[
    "c",
    // "cs",
    // "dart",
    "javascript",
    "kotlin",
    "python",
    "rust",
    "scheme",
    "typescript",
    "lua",
];

/// Launches a new Neovim process in headless mode, executes a list of commands
/// and outputs the result.
fn nvim_exec(cmds: &[&str]) -> Vec<String> {
    let raw_bytes = Command::new("nvim")
        .args(["-u", "NONE", "--headless"])
        .args(cmds.iter().flat_map(|&cmd| ["-c", cmd]))
        .args(["+quit"])
        .output()
        .expect("Couldn't find `nvim` in $PATH")
        .stderr;

    raw_bytes.lines().map(Result::unwrap).collect()
}

/// Returns a treesitter query string for a given language from Neovim's
/// runtime path.
fn get_treesitter_query(lang: &str, query: &str) -> String {
    nvim_exec(&[
        "lua query_files = vim.treesitter.query.get_query_files",
        &format!(r#"lua paths = query_files("{lang}", "{query}")"#),
        "lua for _, path in pairs(paths) do print(path); end",
    ])
    .into_iter()
    .map(|filepath| fs::read_to_string(filepath).unwrap_or_default())
    .collect::<Vec<String>>()
    .join("\n")
}

/// Converts Neovim filetypes to Treesitter language names.
fn to_lang(ft: &str) -> &'_ str {
    match ft {
        "cs" => "c_sharp",
        _ => ft,
    }
}

/// Converts Treesitter languages to the name of the function used to get a
/// `tree_sitter::Language` in the respective crate.
fn to_fn_name(lang: &str) -> &'static str {
    match lang {
        "typescript" => "language_typescript",
        _ => "language",
    }
}

fn main() -> io::Result<()> {
    let out_dir = Path::new("src");

    // Generate the treesitter query files for every filetype.
    for lang in SUPPORTED_FILETYPES.into_iter().map(|&ft| to_lang(ft)) {
        let dir = out_dir.join("queries").join(lang);
        fs::create_dir_all(&dir)?;

        let highlights = dir.join("highlights.scm");
        let injections = dir.join("injections.scm");
        let locals = dir.join("locals.scm");

        fs::write(highlights, get_treesitter_query(lang, "highlights"))?;
        fs::write(injections, get_treesitter_query(lang, "injections"))?;
        fs::write(locals, get_treesitter_query(lang, "locals"))?;
    }

    let match_arms = SUPPORTED_FILETYPES
        .iter()
        .map(|ft| {
            let lang = to_lang(ft);
            format!(
                r#"
            "{ft}" => (
                tree_sitter_{lang}::{}(),
                include_str!("queries/{lang}/highlights.scm"),
                include_str!("queries/{lang}/injections.scm"),
                include_str!("queries/{lang}/locals.scm"),
            ),
                "#,
                to_fn_name(lang)
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let from_filetype = format!(
        r#"use tree_sitter_highlight::HighlightConfiguration;

pub fn config_from_filetype(ft: &str) -> Option<HighlightConfiguration> {{
    let (language, highlights, injections, locals) = match ft {{
        {match_arms}
        _ => return None,
    }};

    HighlightConfiguration::new(language, highlights, injections, locals).ok()
}}"#
    );

    fs::write(out_dir.join("generated.rs"), from_filetype)?;

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/constants.rs");
    println!("cargo:rerun-if-changed=src/highlighter.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");

    Ok(())
}
