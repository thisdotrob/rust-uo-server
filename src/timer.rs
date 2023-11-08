use chrono::prelude::*;
use std::sync::{Arc, Mutex, mpsc};

mod registration_thread;
mod prioritisation_thread;
mod execution_thread;

type CallBack = Arc<dyn Fn() + Send + Sync>;

pub struct Timer {
    name: String,
    repetitions: isize,
    interval: i64,
    next: i64, // TODO rename to `next_tick`?
    callback: CallBack,
}

pub struct TimerArgs {
    pub name: String,
    pub repetitions: isize,
    pub interval: i64,
    pub callback: CallBack,
}

pub fn current_ticks() -> i64 {
    let utc_now = Utc::now();
    utc_now.timestamp_millis()
}

pub fn start() -> mpsc::Sender<TimerArgs> {
    let (register_tx, register_rx) = mpsc::channel::<TimerArgs>();
    let (execute_tx, execute_rx) = mpsc::channel::<Timer>();

    let new_timers: Vec<Timer> = vec![];
    let new_timers = Arc::new(Mutex::new(new_timers));

    registration_thread::spawn(register_rx, Arc::clone(&new_timers));
    prioritisation_thread::spawn(execute_tx, new_timers);
    execution_thread::spawn(execute_rx);

    return register_tx
}
