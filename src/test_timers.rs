use std::sync::mpsc;
use crate::timer::Timer;
use crate::ticks::current_ticks;
use crate::state::{Character, Monster};

pub fn start(timer_register_tx: mpsc::Sender<Timer>) {
    // Start a Character timer that decrements hitpoints by 1 every second for 90 repetitions
    let repetitions = 3;
    let interval = 1000;
    let next = current_ticks() + interval;
    let mut state = Character {
        name: String::from("Bob"),
        hitpoints: 100,
    };
    let callback = Box::new(move || {
        state.hitpoints -= 1;
        println!("Character {} hitpoints are now: {}", state.name, state.hitpoints);
    });

    let timer = Timer { repetitions, interval, next, callback };
    timer_register_tx.send(timer).unwrap();

    // Start a Monster timer that increases anger by 10 every 500ms for 50 repetitions
    let repetitions = 50;
    let interval = 500;
    let next = current_ticks() + interval;
    let mut state = Monster {
        name: String::from("Dave"),
        anger: 0,
    };
    let callback = Box::new(move || {
        state.anger += 10;
        println!("Monster {} anger is now: {}", state.name, state.anger);
    });
    let timer = Timer { repetitions, interval, next, callback };
    timer_register_tx.send(timer).unwrap();
}