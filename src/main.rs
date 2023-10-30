mod timer;
mod cli;

fn main() {
    let (progress_bars, timer_register_tx) = timer::start();
    cli::start(progress_bars, timer_register_tx);
}
