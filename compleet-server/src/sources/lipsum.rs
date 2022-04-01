use async_trait::async_trait;
use serde::Deserialize;

use crate::completion::{Completion, Completions};
use crate::cursor::Cursor;
use crate::source::Source;

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

#[async_trait]
impl Source for Lipsum {
    // Attach to all buffers.
    fn attach(&self, _bufnr: u32) -> bool { true }

    async fn complete(&self, cursor: &Cursor) -> Completions {
        let word_pre = cursor.word_pre();

        if word_pre.is_empty() {
            return Vec::new();
        }

        // Simulate a slow source, this shouldn't block.
        // tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        // std::thread::sleep(std::time::Duration::from_secs(5));

        LOREMS
            .iter()
            .filter(|&&word| word.starts_with(word_pre) && word != word_pre)
            .map(|&word| Completion {
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
            .collect()
    }
}
