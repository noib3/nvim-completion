/// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#completionParams
pub struct CompletionParams {
    text_document: String,
    position: Position,
    context: Option<CompletionContext>,
}

/// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#position
struct Position {
    line: u32,
    character: u32,
}

/// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#completionContext
struct CompletionContext {
    trigger_kind: CompletionTriggerKind,
    trigger_character: Option<char>,
}

/// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#completionTriggerKind
enum CompletionTriggerKind {
    Invoked = 1,
    TriggerCharacter = 2,
    TriggerForIncompleteCompletions = 3,
}
