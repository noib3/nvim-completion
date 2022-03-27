use serde::Deserialize;

use crate::completion::{sources, CompletionSource};

#[derive(Debug, Deserialize)]
pub struct Lipsum {
    #[serde(default = "default_enable")]
    pub enable: bool,
}

impl Default for Lipsum {
    fn default() -> Self {
        let lipsum = sources::Lipsum::new();
        Lipsum {
            enable: lipsum.enable(),
        }
    }
}

fn default_enable() -> bool { sources::Lipsum::new().enable() }
