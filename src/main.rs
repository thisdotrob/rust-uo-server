use std::thread;
use std::time::Duration;

mod timer;
mod ticks;
mod state;
mod test_timers;

fn main() {
    let timer_register_tx = timer::start();
    test_timers::start(timer_register_tx);
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}
