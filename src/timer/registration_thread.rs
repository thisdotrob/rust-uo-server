use super::Timer;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub fn spawn(register_rx: mpsc::Receiver<Timer>, new_timers: Arc<Mutex<Vec<Timer>>>) {
    thread::spawn(move || {
        for timer in register_rx {
            let mut new_timers = new_timers.lock().unwrap();
            new_timers.push(timer);
        }
    });
}
