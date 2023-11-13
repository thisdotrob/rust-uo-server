use std::sync::{Arc, Mutex, mpsc};
use crate::state::{State, StateDelta};

mod registration_thread;
mod prioritisation_thread;
mod execution_thread;

pub struct Timer<T: State> {
    pub repetitions: isize,
    pub interval: i64,
    pub next: i64, // TODO rename to `next_tick`?
    pub state: T,
    pub state_deltas: Vec<StateDelta>,
}

pub trait Callback {
    fn callback(&mut self);
    fn repetitions(&self) -> isize;
    fn next(&self) -> i64;
}

impl<T> Callback for Timer<T> where T: State {
    fn callback(&mut self) {
        self.repetitions -= 1;
        self.next = self.next + self.interval;
        self.state.update_state(&self.state_deltas);
    }

    fn repetitions(&self) -> isize {
        return self.repetitions;
    }

    fn next(&self) -> i64 {
        return self.next;
    }
}

pub fn start() -> mpsc::Sender<Box<dyn Callback + Send>> {
    let (register_tx, register_rx) = mpsc::channel::<Box<dyn Callback + Send>>();
    let (execute_tx, execute_rx) = mpsc::channel::<Box<dyn Callback + Send>>();

    let new_timers: Vec<Box<dyn Callback + Send>> = vec![];
    let new_timers = Arc::new(Mutex::new(new_timers));

    registration_thread::spawn(register_rx, Arc::clone(&new_timers));
    prioritisation_thread::spawn(execute_tx, new_timers);
    execution_thread::spawn(execute_rx, register_tx.clone());

    return register_tx
}
