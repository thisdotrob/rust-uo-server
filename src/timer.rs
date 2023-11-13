use std::sync::{Arc, Mutex, mpsc};

mod registration_thread;
mod prioritisation_thread;
mod execution_thread;

type CallBack = Arc<dyn Fn() + Send + Sync>;

pub struct Timer {
    pub name: String,
    pub repetitions: isize,
    pub interval: i64,
    pub next: i64, // TODO rename to `next_tick`?
    pub callback: CallBack,
}

pub fn start() -> mpsc::Sender<Timer> {
    let (register_tx, register_rx) = mpsc::channel::<Timer>();
    let (execute_tx, execute_rx) = mpsc::channel::<Timer>();

    let new_timers: Vec<Timer> = vec![];
    let new_timers = Arc::new(Mutex::new(new_timers));

    registration_thread::spawn(register_rx, Arc::clone(&new_timers));
    prioritisation_thread::spawn(execute_tx, new_timers);
    execution_thread::spawn(execute_rx);

    return register_tx
}
