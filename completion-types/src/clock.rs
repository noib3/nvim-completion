use std::time::Instant;

/// A clock used to measure the performance of the various stages of the
/// completion pipeline.
#[derive(Clone, Debug)]
pub struct Clock {
    /// There are 3 separate time measurements that we care about, in
    /// chronological order:
    ///
    /// - from the user editing the buffer to a completion source sending back
    ///   its results;
    /// - the time spent resorting the completions (old + new);
    /// - from the client receiving the new sorted completions to the UI being
    ///   updated.
    ///
    /// The first 2 happen core-side, while the client is responsible for the
    /// last one. 3 deltas => 4 ticks.
    times: [Option<Instant>; 4],
}

impl Clock {
    #[cfg(feature = "client")]
    pub fn start() -> Self {
        Self { times: [Some(Instant::now()), None, None, None] }
    }

    #[cfg(feature = "core")]
    pub fn time_source_finished(&mut self) {
        debug_assert!(self.times[1..].iter().all(Option::is_none));
        self.times[1] = Some(Instant::now());
    }

    #[cfg(feature = "core")]
    pub fn time_completions_sorted(&mut self) {
        debug_assert!(self.times[..=1].iter().all(Option::is_some));
        debug_assert!(self.times[2..].iter().all(Option::is_none));
        self.times[2] = Some(Instant::now());
    }

    #[cfg(feature = "client")]
    pub fn time_ui_updated(&mut self) {
        debug_assert!(self.times[..=2].iter().all(Option::is_some));
        debug_assert!(self.times[3].is_none());
        self.times[3] = Some(Instant::now());
    }

    #[cfg(feature = "client")]
    pub fn report(&self) -> [u64; 3] {
        debug_assert!(self.times[..].iter().all(Option::is_some));
        match self.times {
            [Some(start), Some(source), Some(sort), Some(ui)] => [
                (source - start).as_millis() as _,
                (sort - source).as_millis() as _,
                (ui - sort).as_millis() as _,
            ],
            _ => unreachable!(),
        }
    }
}
