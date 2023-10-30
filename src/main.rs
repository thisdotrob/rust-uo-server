use chrono::prelude::*;
use std::thread;
use std::time::Duration;
use std::io;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

#[derive(Debug)]
struct Timer {
    callback: String,
    repetitions: isize,
    interval: i64,
    next: i64, // TODO rename to `next_tick`?
}

#[derive(Debug)]
struct TimerArgs {
    callback: String,
    repetitions: isize,
    interval: i64,
}

fn clear_terminal() {
    print!("{}[2J", 27 as char);
}

fn current_ticks() -> i64 {
    let utc_now = Utc::now();
    utc_now.timestamp_millis()
}

fn main() {
    let m = Arc::new(Mutex::new(MultiProgress::new()));
    let m2 = Arc::clone(&m);

    let (timer_register_tx, timer_register_rx) = mpsc::channel::<TimerArgs>();
    let (timer_execute_tx, timer_execute_rx) = mpsc::channel::<Timer>();

    let _timer_execute_thread = thread::spawn(move || {
        let sty = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        ).unwrap().progress_chars("##-");

        let mut progress_bars: HashMap<String, ProgressBar> = HashMap::new();

        for timer in timer_execute_rx {
            let progress_bar = progress_bars.get(&timer.callback);
            match progress_bar {
                Some(pb) => {
                    pb.inc(1);
                }
                None => {
                    let m = m.lock().unwrap();
                    let total: u64 = timer.repetitions.try_into().unwrap();
                    let pb = m.add(ProgressBar::new(total));
                    pb.set_style(sty.clone());
                    pb.set_message(String::from(&timer.callback));
                    pb.inc(1);
                    progress_bars.insert(String::from(&timer.callback), pb);
                }
            }
        }
    });

    let new_timers: Vec<Timer> = vec![];
    let new_timers_ref_1 = Arc::new(Mutex::new(new_timers));
    let new_timers_ref_2 = Arc::clone(&new_timers_ref_1);

    let _timer_registrar_thread = thread::spawn(move || {
        for timer_args in timer_register_rx {
            let TimerArgs { callback, repetitions, interval } = timer_args;
            let next = current_ticks() + interval;
            let timer = Timer { callback, repetitions, interval, next };
            let mut new_timers = new_timers_ref_1.lock().unwrap();
            new_timers.push(timer);
        }
    });

    let _timer_prioritiser_thread = thread::spawn(move || {
        let mut timers = vec![];

        loop {
            thread::sleep(Duration::from_millis(1));
            {
                let mut new_timers = new_timers_ref_2.lock().unwrap();

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
                timer_execute_tx.send(timer).unwrap();
            }

            timers = next_timers;
        }
    });

    loop {
        clear_terminal();

        println!("Press return to start adding a new timer");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let m = m2.lock().unwrap();

        m.suspend(|| {
            clear_terminal();

            println!("Provide stdin with a string in the following format to register a new timer:");
            println!("name repetitions interval(ms)");
            println!("e.g.: \"timer0 100 50\"");

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let mut split_input = input.split_whitespace();

            let name = split_input.next().unwrap();
            let repetitions = split_input.next().unwrap();
            let repetitions: isize = repetitions.parse().expect("Failed to parse numeric string");
            let interval = split_input.next().unwrap();
            let interval: i64 = interval.parse().expect("Failed to parse numeric string");

            let timer_args = TimerArgs {
                callback: String::from(name), repetitions, interval
            };

            timer_register_tx.send(timer_args).unwrap();
        })
    }
}
