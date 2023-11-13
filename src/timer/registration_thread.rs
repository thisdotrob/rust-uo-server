use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use super::Callback;

pub fn spawn(register_rx: mpsc::Receiver<Box<dyn Callback + Send>>, new_timers: Arc<Mutex<Vec<Box<dyn Callback + Send>>>>) {
    thread::spawn(move || {
        for mut timer in register_rx {
            let mut new_timers = new_timers.lock().unwrap();
            timer.callback();
            new_timers.push(timer);
        }
    });
}
