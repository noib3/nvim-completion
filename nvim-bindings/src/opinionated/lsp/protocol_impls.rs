use super::protocol::{CompletionParams, Position, TextDocumentIdentifier};

impl CompletionParams {
    pub fn new(filename: String, line: u32, character: u32) -> Self {
        Self {
            text_document: TextDocumentIdentifier::new(filename),
            position: Position { line, character },
            context: None,
        }
    }
}

// TODO: have different path delimiter on Windows

impl TextDocumentIdentifier {
    fn new(filepath: String) -> Self {
        Self { uri: format!("file://{filepath}") }
    }
}
