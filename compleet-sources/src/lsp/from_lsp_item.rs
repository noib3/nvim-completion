use bindings::opinionated::lsp::protocol::{
    CompletionItem as LspCompletionItem,
    CompletionItemDocumentation,
    CompletionItemKind,
    CompletionItemTextEdit,
    MarkupKind,
};
use treesitter_highlighter::Highlighter;

use super::constants::{hlgroup, icon};
// use crate::completion_builder::CompletionItemBuilder;
use crate::completion_item::{CompletionItem, CompletionItemBuilder};

impl CompletionItem {
    /// Converts a completion item coming from an Lsp server into our own
    /// `crate::completion_item::CompletionItem`.
    pub fn from_lsp_item(
        lsp_item: LspCompletionItem,
        filetype: &str,
        ts_highlighter: Option<&mut Highlighter>,
    ) -> CompletionItem {
        let text = match lsp_item.text_edit {
            Some(edit) => {
                use CompletionItemTextEdit::*;
                match edit {
                    TextEdit(e) => e.new_text,
                    InsertReplaceEdit(e) => e.new_text,
                }
            },

            None => lsp_item.insert_text.unwrap_or(lsp_item.label),
        };

        let mut builder = CompletionItemBuilder::new(text);

        if let Some(kind) = lsp_item.kind {
            use CompletionItemKind::*;

            let (icon, _hlgroup) = match kind {
                Text => (icon::TEXT, hlgroup::TEXT),
                Method => (icon::METHOD, hlgroup::METHOD),
                Function => (icon::FUNCTION, hlgroup::FUNCTION),
                Constructor => (icon::CONSTRUCTOR, hlgroup::CONSTRUCTOR),
                Field => (icon::FIELD, hlgroup::FIELD),
                Variable => (icon::VARIABLE, hlgroup::VARIABLE),
                Class => (icon::CLASS, hlgroup::CLASS),
                Interface => (icon::INTERFACE, hlgroup::INTERFACE),
                Module => (icon::MODULE, hlgroup::MODULE),
                Property => (icon::PROPERTY, hlgroup::PROPERTY),
                Unit => (icon::UNIT, hlgroup::UNIT),
                Value => (icon::VALUE, hlgroup::VALUE),
                Enum => (icon::ENUM, hlgroup::ENUM),
                Keyword => (icon::KEYWORD, hlgroup::KEYWORD),
                Snippet => (icon::SNIPPET, hlgroup::SNIPPET),
                Color => (icon::COLOR, hlgroup::COLOR),
                File => (icon::FILE, hlgroup::FILE),
                Reference => (icon::REFERENCE, hlgroup::REFERENCE),
                Folder => (icon::FOLDER, hlgroup::FOLDER),
                EnumMember => (icon::ENUM_MEMBER, hlgroup::ENUM_MEMBER),
                Constant => (icon::CONSTANT, hlgroup::CONSTANT),
                Struct => (icon::STRUCT, hlgroup::STRUCT),
                Event => (icon::EVENT, hlgroup::EVENT),
                Operator => (icon::OPERATOR, hlgroup::OPERATOR),
                TypeParameter => {
                    (icon::TYPE_PARAMETER, hlgroup::TYPE_PARAMETER)
                },
            };

            builder.icon(icon);
            // TODO: highlight icon.
            // builder.highlight_icon(hlgroup);
        }

        let (maybe_details, filetype) =
            if let Some(docs) = lsp_item.documentation {
                use CompletionItemDocumentation::*;
                match docs {
                    String(str) => (Some(str), ""),
                    MarkupContent(mkup) => (
                        Some(mkup.value),
                        match mkup.kind {
                            MarkupKind::PlainText => "text",
                            MarkupKind::Markdown => "markdown",
                        },
                    ),
                }
            } else {
                (lsp_item.detail, filetype)
            };

        if let Some(detail) = maybe_details {
            builder.details_text(detail).details_ft(filetype);
        }

        let mut completion = builder.build();

        if let Some(hl) = ts_highlighter {
            completion.highlight_label(hl.highlight(&completion.label));
        }

        completion
    }
}
