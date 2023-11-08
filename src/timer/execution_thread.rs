use std::sync::mpsc;
use std::thread;
use super::Timer;

pub fn spawn(execute_rx: mpsc::Receiver<Timer>) {
    thread::spawn(move || {
        for timer in execute_rx {
            let callback = timer.callback;
            callback();
        }
    });
}
