use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;

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
    pub trigger_kind: CompletionTriggerKind,
    pub trigger_character: Option<char>,
}

#[derive(Debug, Serialize)]
pub enum CompletionTriggerKind {
    Invoked = 1,
    TriggerCharacter = 2,
    TriggerForIncompleteCompletions = 3,
}

#[derive(Debug, Deserialize)]
pub struct ResponseError {
    pub code: ErrorCode,
    pub message: String,
}

#[derive(Debug, Deserialize_repr)]
#[repr(i32)]
pub enum ErrorCode {
    // Defined by JSON RPC.
    ParseError = -32700,
    InvalidRequest = -32600,
    MethodNotFound = -32601,
    InvalidParams = -32602,
    InternalError = -32603,

    // JSON RPC reserved error codes.
    ServerNotInitialized = -32002,
    UnknownErrorCode = -32001,

    // Defined by the protocol.
    RequestFailed = -32803,
    ServerCancelled = -32802,
    ContentModified = -32801,
    RequestCancelled = -32800,
}
