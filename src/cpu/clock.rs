const CLOCK_SPEED: u64 = 4_194_304;

fn hz_to_ns(hz: u64) -> u64 {
    1_000_000_000 / hz
}

pub struct Clock {
    duration: std::time::Duration,
    start: std::time::Instant,
}

impl Clock {
    pub fn sleep_until_next_cycle(&mut self) {
        let elapsed = self.start.elapsed();
        if elapsed < self.duration {
            let delta = self.duration - elapsed;
            std::thread::sleep(delta);
        }
        self.start = std::time::Instant::now();
    }
}

impl Default for Clock {
    fn default() -> Self {
        Clock {
            duration: std::time::Duration::from_nanos(hz_to_ns(CLOCK_SPEED)),
            start: std::time::Instant::now(),
        }
    }
}
