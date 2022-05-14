use once_cell::unsync::OnceCell;

#[derive(Debug)]
pub struct LateInit<T> {
    cell: OnceCell<T>,
}

impl<T> Default for LateInit<T> {
    fn default() -> Self {
        LateInit { cell: OnceCell::default() }
    }
}

impl<T> std::ops::Deref for LateInit<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.cell.get().unwrap()
    }
}

impl<T> LateInit<T> {
    pub fn set(v: T) -> Self {
        let cell = OnceCell::new();
        let _ = cell.set(v);
        Self { cell }
    }
}
