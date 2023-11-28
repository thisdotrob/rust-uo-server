use std::thread;
use std::time::Duration;

mod state;
mod tcp;
mod test_timers;
mod ticks;
mod timer;

fn main() {
    let timer_register_tx = timer::start();
    test_timers::start(timer_register_tx);
    tcp::start();

    loop {
        thread::sleep(Duration::from_secs(60));
    }
}
