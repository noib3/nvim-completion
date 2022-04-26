use bindings::opinionated::lsp::protocol::{
    CompletionItem as LspCompletionItem,
    CompletionItemKind,
};

use super::hlgroups::kind;
use crate::{completion_item::CompletionItemBuilder, prelude::CompletionItem};

impl From<LspCompletionItem> for CompletionItem {
    fn from(lsp_item: LspCompletionItem) -> CompletionItem {
        let mut builder = CompletionItemBuilder::new(lsp_item.label);

        if let Some(kind) = lsp_item.kind {
            use CompletionItemKind::*;

            let (icon, hl_group) = match kind {
                Text => ('', kind::TEXT),
                Method => ('', kind::METHOD),
                Function => ('', kind::FUNCTION),
                Constructor => ('', kind::CONSTRUCTOR),
                Field => ('ﰠ', kind::FIELD),
                Variable => ('', kind::VARIABLE),
                Class => ('ﴯ', kind::CLASS),
                Interface => ('', kind::INTERFACE),
                Module => ('', kind::MODULE),
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
                TypeParameter => (' ', kind::TYPE_PARAMETER),
            };

            builder = builder.icon(icon, Some(hl_group));
        }

        if let Some(details) = lsp_item.detail {
            builder = builder.details(details);
        }

        builder.build()
    }
}
