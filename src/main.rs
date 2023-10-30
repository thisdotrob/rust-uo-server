use chrono::prelude::*;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

// TODO use a smaller int
struct Timer {
    callback: String,
    repetitions: isize,
    interval: i64,
    next: i64, // TODO rename to `next_tick`?
}

fn current_ticks() -> i64 {
    let utc_now = Utc::now();
    let nanos = utc_now.timestamp_nanos_opt().unwrap();
    nanos / 100
}

fn register_timer(callback: &String, repetitions: isize, interval: i64, timers: &mut Vec<Timer>) {
    let next = current_ticks() + interval;
    let timer = Timer { callback: String::from(callback), repetitions, interval, next };
    timers.push(timer);
}

fn main() {
    let (timer_execute_tx, timer_execute_rx) = mpsc::channel::<Timer>();

    let _timer_execute_thread = thread::spawn(move || {
        for timer in timer_execute_rx {
            println!("callback from {}", timer.callback);
        }
    });

    let mut timers = vec![];

    let callback = String::from("hello from callback!");

    register_timer(&callback, 10, 2000, &mut timers);

    loop {
        thread::sleep(Duration::from_millis(1000));
        let mut next_timers = vec![];

        while let Option::Some(timer) = timers.pop() {
            if timer.next > current_ticks() {
                next_timers.push(timer);
                continue;
            } 

            if timer.repetitions == 0 {
                continue;
            } 

            register_timer(&timer.callback, timer.repetitions - 1, timer.interval, &mut next_timers);
            timer_execute_tx.send(timer).unwrap();
        };

        timers = next_timers;
    }
}
