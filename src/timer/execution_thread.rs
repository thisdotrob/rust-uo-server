use std::sync::mpsc;
use std::thread;
use super::Callback;

pub fn spawn(execute_rx: mpsc::Receiver<Box<dyn Callback + Send>>, register_tx: mpsc::Sender<Box<dyn Callback + Send>>) {
    thread::spawn(move || {
        for mut timer in execute_rx {
            timer.callback();
            if timer.repetitions() > 0 {
                register_tx.send(timer).unwrap();
            }
        }
    });
}
