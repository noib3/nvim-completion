use nvim_oxi::api::Buffer;
use ropey::Rope;

pub struct CompletionContext {
    buffer: Buffer,
    rope: Rope,
}
