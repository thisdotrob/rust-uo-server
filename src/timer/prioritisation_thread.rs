use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;
use super::Callback;
use crate::ticks::current_ticks;

pub fn spawn(execute_tx: mpsc::Sender<Box<dyn Callback + Send>>, new_timers: Arc<Mutex<Vec<Box<dyn Callback + Send>>>>) {
    thread::spawn(move || {
        let mut timers: Vec<Box<dyn Callback + Send>> = Vec::new();

        loop {
            thread::sleep(Duration::from_millis(1));
            {
                let mut new_timers = new_timers.lock().unwrap();

                while let Some(timer) = new_timers.pop() {
                    timers.push(timer);
                }
            }

            let mut not_due = vec![];

            let now = current_ticks();

            for timer in timers {
                if timer.next() < now {
                    execute_tx.send(timer).unwrap();
                } else {
                    not_due.push(timer);
                }
            }

            timers = not_due;
        }
    });
}
