use std::io;

mod timer;

fn clear_terminal() {
    print!("{}[2J", 27 as char);
}

fn main() {
    let (progress_bars_ref, timer_register_tx) = timer::start();

    loop {
        clear_terminal();

        println!("Press return to start adding a new timer");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let progress_bars = progress_bars_ref.lock().unwrap();

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

            let timer_args = timer::TimerArgs {
                callback: String::from(name), repetitions, interval
            };

            timer_register_tx.send(timer_args).unwrap();
        })
    }
}
