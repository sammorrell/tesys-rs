use std::time::{Duration, Instant};
use std::{thread};

#[derive(Debug)]
pub struct LoopTimer {
    target_loop_time: Duration, // The time the loop is required to be in miliseconds
    loop_start: Instant,
    loop_end: Instant,
    loop_delay: Duration,
    delay_loop: bool,
}

impl LoopTimer {
    pub fn new(loop_rate: u64) -> LoopTimer {
        // Lets begin by having a duration.
        let dur_ms = ((1.0 / loop_rate as f64) * 1000.0).ceil() as u64;
        let target_loop_time = Duration::from_millis(dur_ms);

        LoopTimer {
            target_loop_time: target_loop_time,
            loop_start: Instant::now(),
            loop_end: Instant::now(),
            loop_delay: Duration::new(0, 0),
            delay_loop: true,
        }
    }

    /// Sets the target rate of the loop in iterations per second.
    pub fn set_loop_rate(&mut self, loop_rate: u64) {
        self.target_loop_time = Duration::from_micros(1000_000 / loop_rate);
    }

    /// Returns the target loop rate in iterations per second.
    pub fn get_target_loop_rate(&self) -> u64 {
        let dur = self.target_loop_time.as_secs() + (self.target_loop_time.subsec_millis()) as u64;
        (1 / dur) as u64
    }

    pub fn get_loop_time(&self) -> Duration {
        self.loop_end.clone() - self.loop_start.clone()
    }

    pub fn get_loop_delay(&self) -> Duration {
        self.loop_delay.clone()
    }

    pub fn start(&mut self) {
        self.loop_start = Instant::now();
    }

    pub fn end(&mut self) {
        // We get the end time of the loop
        self.loop_end = Instant::now();

        // Calculate the time the loop has taken so far.
        let dur = self.get_loop_time();
        self.loop_delay = self.target_loop_time - dur;

        // Delay the loop if required of us
        if self.delay_loop {
            thread::sleep(self.loop_delay);
        }
    }
}
