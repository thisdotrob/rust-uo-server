use crate::timer::TimerArgs;
use indicatif::MultiProgress;
use std::io;
use std::sync::{Arc, Mutex, mpsc};

fn clear_terminal() {
    print!("{}[2J", 27 as char);
}

pub fn start(progress_bars: Arc<Mutex<MultiProgress>>, timer_register_tx: mpsc::Sender<TimerArgs>) {
    loop {
        clear_terminal();

        println!("Press return to start adding a new timer");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let progress_bars = progress_bars.lock().unwrap();

        progress_bars.suspend(|| {
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
