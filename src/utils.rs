use std::time::{Duration, Instant};

pub fn track_time(to_track: Box<dyn FnOnce()>) -> Duration {
    let now = Instant::now();
    to_track();
    now.elapsed()
}

pub struct Puzzle {
    name: String,
    elapsed_time: Duration,
}

impl Puzzle {
    pub fn run(name: &str, task: Box<dyn FnOnce()>) -> Puzzle {
        Puzzle {
            name: name.to_string(),
            elapsed_time: track_time(task),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn elapsed_time(&self) -> &Duration {
        &self.elapsed_time
    }
}
