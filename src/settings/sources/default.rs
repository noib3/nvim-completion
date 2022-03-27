use std::sync::Arc;

use crate::completion::{sources::*, CompletionSource};
use crate::state::Sources;

// TODO: this is annoying. Find a way to loop instead.
pub fn default() -> Sources {
    let mut sources = Vec::new();

    let lipsum = Lipsum::default();
    if lipsum.enable {
        sources.push(Arc::new(lipsum) as Arc<dyn CompletionSource>)
    }

    sources
}
