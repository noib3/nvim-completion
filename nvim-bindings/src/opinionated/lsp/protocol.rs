use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/

pub type DocumentUri = String;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionParams {
    #[serde(flatten)]
    pub text_document_position_params: TextDocumentPositionParams,
    pub context: Option<CompletionContext>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentPositionParams {
    pub text_document: TextDocumentIdentifier,
    pub position: Position,
}

#[derive(Debug, Serialize)]
pub struct TextDocumentIdentifier {
    pub uri: DocumentUri,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize_repr)]
#[repr(u8)]
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

#[derive(Debug, Deserialize_repr, PartialEq)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionItem {
    pub label: String,
    pub label_details: Option<CompletionItemLabelDetails>,
    pub kind: Option<CompletionItemKind>,
    pub tags: Option<Vec<CompletionItemTag>>,
    pub detail: Option<String>,
    pub documentation: Option<CompletionItemDocumentation>,
    pub preselect: Option<bool>,
    pub deprecated: Option<bool>,
    pub sort_text: Option<String>,
    pub filter_text: Option<String>,
    pub insert_text: Option<String>,
    pub insert_text_format: Option<InsertTextFormat>,
    pub insert_text_mode: Option<InsertTextMode>,
    pub text_edit: Option<CompletionItemTextEdit>,
    pub additional_text_edits: Option<Vec<TextEdit>>,
    pub commit_characters: Option<Vec<char>>,
    pub command: Option<Command>,
    pub data: Option<LspAny>,
}

#[derive(Debug, Deserialize)]
pub struct CompletionItemLabelDetails {
    pub detail: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum CompletionItemKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Unit = 11,
    Value = 12,
    Enum = 13,
    Keyword = 14,
    Snippet = 15,
    Color = 16,
    File = 17,
    Reference = 18,
    Folder = 19,
    EnumMember = 20,
    Constant = 21,
    Struct = 22,
    Event = 23,
    Operator = 24,
    TypeParameter = 25,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum CompletionItemTag {
    Deprecated = 1,
}

// This enum is **not** part of the official Lsp protocol.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CompletionItemDocumentation {
    String(String),
    MarkupContent(MarkupContent),
}

#[derive(Debug, Deserialize)]
pub struct MarkupContent {
    pub kind: MarkupKind,
    pub value: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarkupKind {
    PlainText,
    Markdown,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum InsertTextFormat {
    PlainText = 1,
    Snippet = 2,
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
pub enum InsertTextMode {
    AsIs = 1,
    AdjustIndentation = 2,
}

// This enum is **not** part of the official Lsp protocol.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CompletionItemTextEdit {
    TextEdit(TextEdit),
    InsertReplaceEdit(InsertReplaceEdit),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}

#[derive(Debug, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertReplaceEdit {
    pub new_text: String,
    pub insert: Range,
    pub replace: Range,
}

#[derive(Debug, Deserialize)]
pub struct Command {
    pub title: String,
    pub command: String,
    pub arguments: Option<Vec<LspAny>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum LspAny {
    LspObject(std::collections::HashMap<String, LspAny>),
    LspArray(Vec<LspAny>),
    String(String),
    Integer(i64),
    Decimal(f64),
    Boolean(bool),
    Null,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionList {
    pub is_incomplete: bool,
    pub items: Vec<CompletionItem>,
}

// This enum is **not** part of the official Lsp protocol.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CompletionResponse {
    List(CompletionList),
    Array(Vec<CompletionItem>),
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum PositionEncodingKind {
    #[serde(rename = "utf-8")]
    Utf8,

    #[serde(rename = "utf-16")]
    Utf16,

    #[serde(rename = "utf-32")]
    Utf32,
}
