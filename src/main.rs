use std::thread;
use std::time::Duration;

mod timer;
mod ticks;
mod state;
mod test_timers;
mod tcp;

fn main() {
    let timer_register_tx = timer::start();
    test_timers::start(timer_register_tx);
    tcp::start();

    loop {
        thread::sleep(Duration::from_secs(60));
    }
}
