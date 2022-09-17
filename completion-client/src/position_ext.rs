use completion_types::Position;
use nvim_oxi::{self as nvim, api::Buffer};

pub(crate) trait PositionExt: Sized {
    fn current() -> nvim::Result<Self> {
        let row = 0;
        let col = 1;
        let buf = Buffer::current();

        <Self as PositionExt>::from_row_col_buf(row, col, &buf)
    }

    fn from_row_col_buf(
        row: usize,
        col: usize,
        buffer: &Buffer,
    ) -> nvim::Result<Self>;
}

impl PositionExt for Position {
    fn current() -> nvim::Result<Self> {
        todo!()
    }

    fn from_row_col_buf(
        row: usize,
        col: usize,
        buffer: &Buffer,
    ) -> nvim::Result<Self> {
        let line = buffer
            .get_lines(row, row + 1, true)?
            .next()
            .unwrap()
            .to_string_lossy()
            .to_string();

        Ok(Self { row: row as _, character: col as _, line })
    }
}
