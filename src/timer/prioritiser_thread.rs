use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;
use super::{Timer, current_ticks};

pub fn spawn(execute_tx: mpsc::Sender<Timer>, new_timers: Arc<Mutex<Vec<Timer>>>) {
    let _timer_prioritiser_thread = thread::spawn(move || {
        let mut timers = vec![];

        loop {
            thread::sleep(Duration::from_millis(1));
            {
                let mut new_timers = new_timers.lock().unwrap();

                while let Some(timer) = new_timers.pop() {
                    timers.push(timer);
                }
            }

            let mut next_timers = vec![];

            let now = current_ticks();

            while let Some(timer) = timers.pop() {
                if timer.next > now {
                    next_timers.push(timer);
                    continue
                }

                if timer.repetitions == 0 {
                    continue
                }

                let next_timer = Timer {
                    callback: String::from(&timer.callback),
                    repetitions: timer.repetitions - 1,
                    interval: timer.interval,
                    next: timer.next + timer.interval,
                };

                next_timers.push(next_timer);
                execute_tx.send(timer).unwrap();
            }

            timers = next_timers;
        }
    });
}
