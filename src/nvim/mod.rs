mod buffer;
mod nvim_buf_add_highlight;
mod nvim_buf_clear_namespace;
mod nvim_buf_set_lines;
mod nvim_create_buf;

pub use buffer::Buffer;
use nvim_buf_add_highlight::nvim_buf_add_highlight;
use nvim_buf_clear_namespace::nvim_buf_clear_namespace;
use nvim_buf_set_lines::nvim_buf_set_lines;
use nvim_create_buf::nvim_create_buf;
