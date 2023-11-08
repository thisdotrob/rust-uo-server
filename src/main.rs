use indicatif::MultiProgress;
use std::sync::{Arc, Mutex};

mod timer;
mod cli;

fn main() {
    let progress_bars = Arc::new(Mutex::new(MultiProgress::new()));

    let timer_register_tx = timer::start(Arc::clone(&progress_bars));

    cli::start(progress_bars, timer_register_tx);
}
