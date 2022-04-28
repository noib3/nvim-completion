use bindings::opinionated::lsp::protocol::{
    CompletionItem as LspCompletionItem,
    CompletionItemKind,
    CompletionItemTextEdit,
};

use super::hlgroups::kind;
use crate::{completion_item::CompletionItemBuilder, prelude::CompletionItem};

impl From<LspCompletionItem> for CompletionItem {
    fn from(lsp_item: LspCompletionItem) -> CompletionItem {
        // if lsp_item.label.starts_with("self") {
        //     println!("{:?}", lsp_item);
        // }

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

            let (icon, hl_group) = match kind {
                Text => ('', kind::TEXT),
                Method => ('', kind::METHOD),
                Function => ('', kind::FUNCTION),
                Constructor => ('', kind::CONSTRUCTOR),
                Field => ('ﰠ', kind::FIELD),
                Variable => ('', kind::VARIABLE),
                Class => ('ﴯ', kind::CLASS),
                Interface => ('', kind::INTERFACE),
                Module => ('', kind::MODULE),
                Property => ('ﰠ', kind::PROPERTY),
                Unit => ('塞', kind::UNIT),
                Value => ('', kind::VALUE),
                Enum => ('', kind::ENUM),
                Keyword => ('', kind::KEYWORD),
                Snippet => ('', kind::SNIPPET),
                Color => ('', kind::COLOR),
                File => ('', kind::FILE),
                Reference => ('', kind::REFERENCE),
                Folder => ('', kind::FOLDER),
                EnumMember => ('', kind::ENUM_MEMBER),
                Constant => ('', kind::CONSTANT),
                Struct => ('פּ', kind::STRUCT),
                Event => ('', kind::EVENT),
                Operator => ('', kind::OPERATOR),
                TypeParameter => ('', kind::TYPE_PARAMETER),
            };

            builder = builder.icon(icon, Some(hl_group));
        }

        if let Some(details) = lsp_item.detail {
            // TODO: detect filetype
            builder = builder.details(details).details_ft("rust".into());
        }

        builder.build()
    }
}
