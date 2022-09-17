use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Clock {
    start: Instant,
}

impl Clock {
    pub fn start() -> Self {
        Self { start: Instant::now() }
    }
}
