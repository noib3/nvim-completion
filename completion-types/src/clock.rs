use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Clock {
    times: [u128; 3],
    start: Instant,
}

impl Clock {
    pub fn start() -> Self {
        Self { times: [0; 3], start: Instant::now() }
    }

    pub fn time_source(&mut self) {
        let from_start = self.start.elapsed().as_millis();
        self.times[0] = from_start;
    }

    pub fn time_sorting(&mut self) {
        let from_start = self.start.elapsed().as_millis();
        self.times[1] = from_start - self.times[0];
    }

    pub fn time_displaying(&mut self) {
        let from_start = self.start.elapsed().as_millis();
        self.times[2] = from_start - self.times[1] - self.times[0];
    }
}
