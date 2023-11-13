use std::sync::mpsc;
use crate::timer::{Timer, Callback};
use crate::ticks::current_ticks;
use crate::state::{StateDelta, Character, Monster};

pub fn start(timer_register_tx: mpsc::Sender<Box<dyn Callback + Send>>) {
    // Start a Character timer that decrements hitpoints by 1 every second for 90 repetitions
    let repetitions = 90;
    let interval = 1000;
    let next = current_ticks() + interval;
    let state = Character {
        name: String::from("Bob"),
        hitpoints: 100,
    };
    let state_deltas = vec![
        StateDelta { property: String::from("hitpoints"), delta: -1 },
    ];
    let timer = Timer { repetitions, interval, next, state, state_deltas };
    timer_register_tx.send(Box::new(timer)).unwrap();

    // Start a Monster timer that increases anger by 10 every 500ms for 50 repetitions
    let repetitions = 50;
    let interval = 500;
    let next = current_ticks() + interval;
    let state = Monster {
        name: String::from("Dave"),
        anger: 0,
    };
    let state_deltas = vec![
        StateDelta { property: String::from("anger"), delta: 10 },
    ];
    let timer = Timer { repetitions, interval, next, state, state_deltas };
    timer_register_tx.send(Box::new(timer)).unwrap();
}
