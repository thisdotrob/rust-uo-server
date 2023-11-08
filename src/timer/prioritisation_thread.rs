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

            let mut not_due = vec![];

            let now = current_ticks();

            for timer in timers {
                if timer.next < now {
                    if timer.repetitions > 1 {
                        println!("next before: {}", timer.next);
                        let next_repetition = Timer {
                            name: String::from(&timer.name),
                            repetitions: timer.repetitions - 1,
                            interval: timer.interval,
                            next: timer.next + timer.interval,
                        };
                        println!("next after: {}", next_repetition.next);
                        not_due.push(next_repetition);
                    }
                    execute_tx.send(timer).unwrap();
                } else {
                    not_due.push(timer);
                }
            }

            timers = not_due;
        }
    });
}
