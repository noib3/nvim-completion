use std::fmt;

/// Subset of `:h lsp-method` relevant to code completion.
pub enum LspMethod {
    Completion,
}

impl fmt::Display for LspMethod {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        use LspMethod::*;

        formatter.write_str(&format!(
            "textDocument/{}",
            match self {
                Completion => "completion",
            }
        ))
    }
}
