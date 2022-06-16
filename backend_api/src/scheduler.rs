use std::time::Duration;

use actix::{clock, Arbiter};
use futures::StreamExt;
use tokio_stream::wrappers::IntervalStream;

pub struct Scheduler {
    arbiter: Arbiter,
}

impl Scheduler {
    pub fn new() -> Self {
        return Self {
            arbiter: Arbiter::new(),
        };
    }

    pub fn run<F, R>(&mut self, interval: Duration, mut task: F)
    where
        F: FnMut() -> R + Send + 'static,
        R: std::future::Future<Output = ()> + Send + 'static,
    {
        let future =
            IntervalStream::new(clock::interval(interval)).for_each_concurrent(2, move |_| task());

        self.arbiter.spawn(future);
    }
}

impl Drop for Scheduler {
    fn drop(&mut self) {
        self.arbiter.stop();
    }
}
