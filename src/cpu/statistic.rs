use std::time::{Duration, Instant};

pub struct Statistic {
  pub time: Duration, // execute time accumulate
  pub count: u64,     // inst count
}

impl Statistic {
  pub fn new() -> Statistic {
    Statistic {
      time: Duration::new(0, 0),
      count: 0,
    }
  }

  pub fn start_timer(&mut self) -> Instant {
    Instant::now()
  }

  pub fn stop_timer(&mut self, start_timer: Instant) {
    self.time += start_timer.elapsed();
  }

  pub fn inc_count(&mut self) {
    self.count += 1;
  }
}