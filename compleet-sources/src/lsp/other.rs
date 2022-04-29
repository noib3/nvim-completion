use bindings::opinionated::lsp::protocol::{
    CompletionItem as LspCompletionItem,
    CompletionItemDocumentation,
    CompletionItemKind,
    CompletionItemTextEdit,
    MarkupKind,
};

use super::constants::{hlgroup, icon};
use crate::prelude::CompletionItem;

impl CompletionItem {
    pub fn from_lsp(
        lsp_item: LspCompletionItem,
        language: &str,
    ) -> CompletionItem {
        let mut completion = CompletionItem::default();

        completion.text = match lsp_item.text_edit {
            Some(edit) => {
                use CompletionItemTextEdit::*;
                match edit {
                    TextEdit(e) => e.new_text,
                    InsertReplaceEdit(e) => e.new_text,
                }
            },

            None => lsp_item.insert_text.unwrap_or(lsp_item.label),
        };

        if let Some(kind) = lsp_item.kind {
            use CompletionItemKind::*;

            let (icon, hl_group) = match kind {
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

            completion.icon = Some(icon.to_string());
            completion.highlight_icon(hl_group);
        }

        let (details, filetype) = if let Some(docs) = lsp_item.documentation {
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
            (lsp_item.detail, language)
        };

        if let Some(det) = details {
            completion.set_details(det, filetype.into());
        }

        completion
    }
}
