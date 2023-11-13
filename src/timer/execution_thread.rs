use std::sync::mpsc;
use std::thread;
use super::Timer;

pub fn spawn(execute_rx: mpsc::Receiver<Timer>, register_tx: mpsc::Sender<Timer>) {
    thread::spawn(move || {
        for mut timer in execute_rx {
            (timer.callback)();
            timer.repetitions -= 1;

            if timer.repetitions > 0 {
                timer.next = timer.next + timer.interval;
                register_tx.send(timer).unwrap();
            }
        }
    });
}
