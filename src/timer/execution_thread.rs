use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use super::Timer;

pub fn spawn(execute_rx: mpsc::Receiver<Timer>, progress_bars: Arc<Mutex<MultiProgress>>) {
    thread::spawn(move || {
        let sty = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        ).unwrap().progress_chars("##-");

        let mut progress_bars_lookup: HashMap<String, ProgressBar> = HashMap::new();

        for timer in execute_rx {
            let progress_bar = progress_bars_lookup.get(&timer.name);
            match progress_bar {
                Some(pb) => {
                    pb.inc(1);
                    println!("incremented existing progress bar");
                }
                None => {
                    let progress_bars = progress_bars.lock().unwrap();
                    let total: u64 = timer.repetitions.try_into().unwrap();
                    let pb = progress_bars.add(ProgressBar::new(total));
                    pb.set_style(sty.clone());
                    pb.set_message(String::from(&timer.name));
                    pb.inc(1);
                    println!("incremented new progress bar");
                    progress_bars_lookup.insert(String::from(&timer.name), pb);
                }
            }
        }
    });
}
