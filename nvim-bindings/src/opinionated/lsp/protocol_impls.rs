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

impl TextDocumentIdentifier {
    fn new(filepath: String) -> Self {
        Self { uri: format!("file://{filepath}") }
    }
}
