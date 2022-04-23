use serde::Serialize;

/// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/

type DocumentUri = String;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionParams {
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
    pub context: Option<CompletionContext>,
}

#[derive(Debug, Serialize)]
pub struct TextDocumentIdentifier {
    pub uri: DocumentUri,
}

#[derive(Debug, Serialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionContext {
    trigger_kind: CompletionTriggerKind,
    trigger_character: Option<char>,
}

#[derive(Debug, Serialize)]
pub enum CompletionTriggerKind {
    Invoked = 1,
    TriggerCharacter = 2,
    TriggerForIncompleteCompletions = 3,
}
