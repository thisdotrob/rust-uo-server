use std::io::{Write, stdout};
use std::io;
use crossterm::{QueueableCommand, cursor};
use std::sync::{Arc, mpsc};
use crate::timer::Timer;
use crate::ticks::current_ticks;

fn clear_terminal() {
    print!("{}[2J", 27 as char);
}

pub fn start(timer_register_tx: mpsc::Sender<Timer>) {
    let mut row = 0;

    loop {
         clear_terminal();

        println!("Press return to start adding a new timer");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

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

        let state = format!("Some state given to the {} timer's callback", name);

        let callback: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
            let mut stdout = stdout();

            stdout.queue(cursor::SavePosition).expect("Error saving position");
            stdout.queue(cursor::MoveTo(0, row)).expect("Error moving cursor");
            let ticks = current_ticks();
            print!("{} - {}", state, ticks);
            stdout.queue(cursor::RestorePosition).expect("Error restoring position");
            stdout.flush().expect("Error flushing");
        });

        row += 1;

        let next = current_ticks() + interval;

        let timer_args = Timer {
            name: String::from(name),
            repetitions,
            interval,
            callback,
            next,
        };

        timer_register_tx.send(timer_args).unwrap();
    }
}
