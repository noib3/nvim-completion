use bindings::opinionated::lsp::protocol::{
    CompletionItem as LspCompletionItem,
    CompletionItemKind,
};

use crate::{completion_item::CompletionItemBuilder, prelude::CompletionItem};

impl From<LspCompletionItem> for CompletionItem {
    fn from(lsp_item: LspCompletionItem) -> CompletionItem {
        let mut builder = CompletionItemBuilder::new(lsp_item.label);

        if let Some(kind) = lsp_item.kind {
            use CompletionItemKind::*;
            let icon = match kind {
                Text => '',
                Method => '',
                Function => '',
                Constructor => '',
                Field => 'ﰠ',
                Variable => '',
                Class => 'ﴯ',
                Interface => '',
                Module => '',
                Property => 'ﰠ',
                Unit => '塞',
                Value => '',
                Enum => '',
                Keyword => '',
                Snippet => '',
                Color => '',
                File => '',
                Reference => '',
                Folder => '',
                EnumMember => '',
                Constant => '',
                Struct => 'פּ',
                Event => '',
                Operator => '',
                TypeParameter => ' ',
            };

            builder = builder.icon(icon);
        }

        if let Some(details) = lsp_item.detail {
            builder = builder.details(details);
        }

        builder.build()
    }
}
