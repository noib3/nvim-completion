use bindings::opinionated::lsp::protocol::{
    CompletionContext,
    CompletionParams,
    CompletionTriggerKind,
    Position,
    PositionEncodingKind,
    TextDocumentIdentifier,
    TextDocumentPositionParams,
};
use bindings::opinionated::Buffer;
use url::Url;
use widestring::ustring::U32String;

use crate::prelude::Cursor;

pub(super) fn make_completion_params(
    buffer: &Buffer,
    cursor: &Cursor,
    encoding: PositionEncodingKind,
) -> CompletionParams {
    let text_document_position_params =
        make_position_params(buffer, cursor, encoding);

    let context = Some(CompletionContext {
        trigger_kind: CompletionTriggerKind::Invoked,
        trigger_character: None,
    });

    CompletionParams { text_document_position_params, context }
}

#[inline]
fn make_position_params(
    buffer: &Buffer,
    cursor: &Cursor,
    encoding: PositionEncodingKind,
) -> TextDocumentPositionParams {
    TextDocumentPositionParams {
        text_document: make_text_document(buffer),
        position: make_position(cursor, encoding),
    }
}

#[inline]
fn make_position(cursor: &Cursor, encoding: PositionEncodingKind) -> Position {
    let line = cursor.row as u32;

    // Only include the text up to the current word boundary when computing the
    // horizontal position. E.g., if the line if `self.foo|`, only consider
    // `self.|`. TODO: explain why.
    let text = {
        let bytes_boundary = cursor.word_bytes_pre(None);
        &cursor.line[..(cursor.bytes - bytes_boundary)]
    };

    let character = match encoding {
        PositionEncodingKind::Utf8 => text.len(),
        PositionEncodingKind::Utf16 => text.encode_utf16().count(),
        PositionEncodingKind::Utf32 => U32String::from(text).len(),
    } as u32;

    Position { line, character }
}

#[inline]
fn make_text_document(buffer: &Buffer) -> TextDocumentIdentifier {
    TextDocumentIdentifier {
        uri: Url::from_file_path(&buffer.filepath)
            .expect("can this fail?")
            .into(),
    }
}
