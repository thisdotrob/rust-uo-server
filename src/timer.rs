use chrono::prelude::*;
use indicatif::MultiProgress;
use std::sync::{Arc, Mutex, mpsc};

mod registrar_thread;
mod prioritiser_thread;
mod executor_thread;

#[derive(Debug)]
pub struct Timer {
    name: String,
    repetitions: isize,
    interval: i64,
    next: i64, // TODO rename to `next_tick`?
}

#[derive(Debug)]
pub struct TimerArgs {
    pub name: String,
    pub repetitions: isize,
    pub interval: i64,
}

fn current_ticks() -> i64 {
    let utc_now = Utc::now();
    utc_now.timestamp_millis()
}

pub fn start() -> (Arc<Mutex<MultiProgress>>, mpsc::Sender<TimerArgs>) {
    let (register_tx, register_rx) = mpsc::channel::<TimerArgs>();
    let (execute_tx, execute_rx) = mpsc::channel::<Timer>();

    let new_timers: Vec<Timer> = vec![];
    let new_timers = Arc::new(Mutex::new(new_timers));

    let progress_bars = Arc::new(Mutex::new(MultiProgress::new()));

    registrar_thread::spawn(register_rx, Arc::clone(&new_timers));
    prioritiser_thread::spawn(execute_tx, new_timers);
    executor_thread::spawn(execute_rx, Arc::clone(&progress_bars));

    return (progress_bars, register_tx)
}
