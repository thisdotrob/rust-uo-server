mod timer;
mod cli;
mod ticks;

fn main() {
    let timer_register_tx = timer::start();
    cli::start(timer_register_tx);
}
