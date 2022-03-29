use mlua::prelude::LuaResult;
use neovim::Api;
use serde::Deserialize;

use crate::completion::{CompletionItem, CompletionSource, Cursor};

const LOREM_IPSUM: [&'static str; 12] = [
    "Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Ut purus elit,",
    "vestibulum ut, placerat ac, adipiscing vitae, felis. Curabitur dictum \
     gravida",
    "mauris. Nam arcu libero, nonummy eget, consectetuer id, vulputate a, \
     magna.",
    "Donec vehicula augue eu neque. Pellentesque habitant morbi tristique \
     senectus",
    "et netus et malesuada fames ac turpis egestas. Mauris ut leo. Cras \
     viverra",
    "metus rhoncus sem. Nulla et lectus vestibulum urna fringilla ultrices.",
    "Phasellus eu tellus sit amet tortor gravida placerat. Integer sapien est",
    "iaculis in, pretium quis, viverra ac, nunc. Praesent eget sem vel leo \
     ultrices",
    "bibendum. Aenean faucibus. Morbi dolor nulla, malesuada eu, pulvinar \
     at, mollis",
    "ac, nulla. Curabitur auctor semper nulla. Donec varius orci eget risus. \
     Duis",
    "nibh mi, congue eu, accumsan eleifend, sagittis quis, diam. Duis eget \
     orci sit",
    "amet orci dignissim",
];

const LOREMS: [&'static str; 128] = [
    "Lorem",
    "ipsum",
    "dolor",
    "sit",
    "amet",
    "consectetuer",
    "adipiscing",
    "elit",
    "Ut",
    "purus",
    "elit",
    "vestibulum",
    "ut",
    "placerat",
    "ac",
    "adipiscing",
    "vitae",
    "felis",
    "Curabitur",
    "dictum",
    "gravida",
    "mauris",
    "Nam",
    "arcu",
    "libero",
    "nonummy",
    "eget",
    "consectetuer",
    "id",
    "vulputate",
    "a",
    "magna",
    "Donec",
    "vehicula",
    "augue",
    "eu",
    "neque",
    "Pellentesque",
    "habitant",
    "morbi",
    "tristique",
    "senectus",
    "et",
    "netus",
    "et",
    "malesuada",
    "fames",
    "ac",
    "turpis",
    "egestas",
    "Mauris",
    "ut",
    "leo",
    "Cras",
    "viverra",
    "metus",
    "rhoncus",
    "sem",
    "Nulla",
    "et",
    "lectus",
    "vestibulum",
    "urna",
    "fringilla",
    "ultrices",
    "Phasellus",
    "eu",
    "tellus",
    "sit",
    "amet",
    "tortor",
    "gravida",
    "placerat",
    "Integer",
    "sapien",
    "est",
    "iaculis",
    "in",
    "pretium",
    "quis",
    "viverra",
    "ac",
    "nunc",
    "Praesent",
    "eget",
    "sem",
    "vel",
    "leo",
    "ultrices",
    "bibendum",
    "Aenean",
    "faucibus",
    "Morbi",
    "dolor",
    "nulla",
    "malesuada",
    "eu",
    "pulvinar",
    "at",
    "mollis",
    "ac",
    "nulla",
    "Curabitur",
    "auctor",
    "semper",
    "nulla",
    "Donec",
    "varius",
    "orci",
    "eget",
    "risus",
    "Duis",
    "nibh",
    "mi",
    "congue",
    "eu",
    "accumsan",
    "eleifend",
    "sagittis",
    "quis",
    "diam",
    "Duis",
    "eget",
    "orci",
    "sit",
    "amet",
    "orci",
    "dignissim",
];

#[derive(Debug, Deserialize)]
pub struct Lipsum {
    pub enable: bool,
}

impl Default for Lipsum {
    fn default() -> Self { Lipsum { enable: false } }
}

impl CompletionSource for Lipsum {
    fn attach(&self, _: &Api, _: u32) -> LuaResult<bool> { Ok(true) }

    fn complete(
        &self,
        _: &Api,
        cursor: &Cursor,
    ) -> LuaResult<Vec<CompletionItem>> {
        let word_pre = cursor.word_pre();

        if word_pre.is_empty() {
            return Ok(Vec::new());
        }

        // Simulate a slow source, this shouldn't block.
        // std::thread::sleep(std::time::Duration::from_secs(2));

        Ok(LOREMS
            .iter()
            .filter(|&&word| word.starts_with(word_pre) && word != word_pre)
            .map(|&word| CompletionItem {
                details: Some(
                    LOREM_IPSUM.map(|word| word.to_string()).to_vec(),
                ),
                format: format!(" {}", word),
                hl_ranges: vec![(
                    1..word_pre.len() + 1,
                    "CompleetMenuMatchingChars",
                )],
                matched_bytes: word_pre.len() as u32,
                source: "Lipsum",
                text: word.to_string(),
            })
            .collect())
    }
}
