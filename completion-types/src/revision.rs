/// Uniquely identifies an edit in time.
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Revision(u32);

impl Revision {
    pub fn advance(&mut self) {
        self.0 += 1;
    }
}
