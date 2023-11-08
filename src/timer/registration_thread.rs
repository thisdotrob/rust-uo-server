use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use super::{Timer, TimerArgs, current_ticks};

pub fn spawn(register_rx: mpsc::Receiver<TimerArgs>, new_timers: Arc<Mutex<Vec<Timer>>>) {
    thread::spawn(move || {
        for timer_args in register_rx {
            let TimerArgs { name, repetitions, interval, callback } = timer_args;
            let next = current_ticks() + interval;
            let timer = Timer { name, repetitions, interval, next, callback };
            let mut new_timers = new_timers.lock().unwrap();
            new_timers.push(timer);
        }
    });
}
