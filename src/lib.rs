use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};
use std::collections::VecDeque;
use std::error::Error;
use std::time::{Duration, Instant};
use structopt::StructOpt;

// Define struct to retain settings.
#[derive(Debug, StructOpt)]
#[structopt(name = "yatti", about = "Yet another TapTempo implementation.")]
pub struct Config {
    /// Set the precision of the processed tempo (max: 5 digits after the decimal point).
    #[structopt(short, long, default_value = "0")]
    pub precision: u8,
    /// Set the time (in sec) before the calculation resets to 0.
    #[structopt(short, long, default_value = "5")]
    pub reset_time: u32,
    /// Set the sample size needed to process the tempo.
    #[structopt(short, long, default_value = "5")]
    pub sample_size: u32,
}

// Run the calculations from keys hits.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut registered_hits = RegisteredHits::new(config.sample_size as usize)?;

    // Read key hits continuously until user hits 'q' or 'Esc'.
    loop {
        // Enable raw mode to directly receive user inputs rather than line-buffered.
        terminal::enable_raw_mode()?;
        // Block waiting for any event.
        match event::read()? {
            // Filter event to keep only key hits events.
            Event::Key(event) => match event.code {
                // Check which key was hit.
                KeyCode::Char(c) => match c {
                    // Quit if 'q' was hit.
                    'q' => break,
                    // Register an hit for any other character key (including space).
                    _ => registered_hits.new_hit(),
                },
                // Also register an hit on 'Enter' key.
                KeyCode::Enter => registered_hits.new_hit(),
                // Quit if 'Esc' was hit (easier for beginners, battle-tested on beloved Mom).
                KeyCode::Esc => break,
                // Continue looping on any other non-character key hit.
                _ => continue,
            },
            // Continue looping on any other non-key event (such as resizing or mouse).
            _ => continue,
        }
        // Disable raw mode to display processing infos to user.
        terminal::disable_raw_mode()?;

        // Display tempo information to the user.
        if let Some(duration) = registered_hits.next() {
            println!(
                "[TEMPO] {:.precision$} BPM",
                process_tempo(duration),
                precision = config.precision as usize
            )
        } else {
            println!("[INFO] hit any key again to run tempo processing...")
        }
    }

    // Disable raw mode again, since it was not disabled by breaking the loop to exit.
    terminal::disable_raw_mode()?;

    Ok(())
}
// Process the tempo in BPM unit.
fn process_tempo(duration: Duration) -> f64 {
    (1_f64 / duration.as_secs_f64()) * 60_f64
}

#[derive(Debug, PartialEq)]
struct RegisteredHits {
    hits: VecDeque<Instant>,
    sample_size: usize,
}

impl RegisteredHits {
    fn new(sample_size: usize) -> Result<RegisteredHits, &'static str> {
        // Check the given sample_size is not lower than or equal to one, since it will
        // be impossible to process the duration on zero or one elements.
        if sample_size <= 1 {
            Err("sample size must be at least two.")
        } else {
            let hits: VecDeque<Instant> = VecDeque::with_capacity(sample_size);
            Ok(RegisteredHits { hits, sample_size })
        }
    }

    fn new_hit(&mut self) {
        // Register the hit time as soon as possible.
        self.hits.push_back(Instant::now());
        // Remove the oldest time stamp if sample size is over its maximum.
        if self.hits.len() > self.sample_size {
            self.hits.pop_front();
        }
    }

    fn reset_hits(&mut self) {
        self.hits.clear();
    }
}

impl Iterator for RegisteredHits {
    type Item = Duration;

    // Yield the duration since the two last key hits.
    fn next(&mut self) -> Option<Self::Item> {
        if self.hits.len() <= 1 {
            None
        } else {
            // Process the elapsed time between key hits.
            let duration = self
                .hits
                .back()?
                .duration_since(*self.hits.front()?)
                .checked_div(self.hits.len() as u32 - 1);

            duration
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registered_hits_ok_sample_size() {
        let registered_hits = RegisteredHits::new(10).unwrap();
        assert_eq!(registered_hits.sample_size, 10);
    }

    #[test]
    fn registered_hits_ok_queue_len() {
        let registered_hits = RegisteredHits::new(10).unwrap();
        assert!(registered_hits.hits.capacity() >= 10);
        assert_eq!(registered_hits.hits.len(), 0);
    }

    #[test]
    #[should_panic]
    fn registered_hits_ko_sample_size_le_1_unwrapped() {
        let _registered_hits = RegisteredHits::new(1).unwrap();
    }

    #[test]
    fn registered_hits_ko_sample_size_le_1() {
        let registered_hits = RegisteredHits::new(1);
        assert_eq!(registered_hits, Err("sample size must be at least two."));
    }

    #[test]
    fn register_new_hit_queue_not_full() {
        let mut registered_hits = RegisteredHits::new(10).unwrap();
        let number_hits = registered_hits.hits.len();
        registered_hits.new_hit();
        assert_eq!(registered_hits.hits.len(), number_hits + 1);
    }

    #[test]
    fn register_new_hit_queue_full() {
        let mut registered_hits = RegisteredHits::new(5).unwrap();
        for _ in 0..10 {
            registered_hits.new_hit();
        }
        let number_hits = registered_hits.hits.len();
        registered_hits.new_hit();
        assert_eq!(registered_hits.hits.len(), number_hits);
    }

    #[test]
    #[should_panic]
    fn registered_hits_iter_next_ko_with_le_1_sample_value() {
        let mut registered_hits = RegisteredHits::new(5).unwrap();
        registered_hits.new_hit();
        registered_hits.next().unwrap();
    }

    #[test]
    fn registered_hits_iter_next_ok() {
        let mut registered_hits = RegisteredHits::new(5).unwrap();
        for _ in 0..2 {
            registered_hits.new_hit();
        }
        registered_hits.next().unwrap();
    }

    #[test]
    fn registered_hits_iter_next_ok_with_sample_values() {
        use std::thread;
        let mut registered_hits = RegisteredHits::new(5).unwrap();
        registered_hits.new_hit();
        thread::sleep(Duration::from_millis(50));
        for _ in 0..9 {
            registered_hits.new_hit();
            thread::sleep(Duration::from_millis(50));
            let duration = registered_hits.next().unwrap();
            assert_eq!(duration.as_millis(), Duration::from_millis(50).as_millis());
        }
    }

    #[test]
    fn registered_hits_reset_ok_len_0() {
        let mut registered_hits = RegisteredHits::new(5).unwrap();
        for _ in 0..10 {
            registered_hits.new_hit();
        }
        registered_hits.reset_hits();
        assert!(registered_hits.hits.is_empty());
    }

    #[test]
    fn registered_hits_reset_ok_capacity_ge_sample_size() {
        let sample_size: usize = 5;
        let mut registered_hits = RegisteredHits::new(sample_size).unwrap();
        for _ in 0..10 {
            registered_hits.new_hit();
        }
        registered_hits.reset_hits();
        assert!(registered_hits.hits.capacity() >= sample_size);
    }

    #[test]
    fn registered_hits_ok_len_0_no_hits() {
        let mut registered_hits = RegisteredHits::new(5).unwrap();
        registered_hits.reset_hits();
        assert!(registered_hits.hits.is_empty());
    }

    #[test]
    fn registered_hits_reset_ok_capacity_ge_sample_size_no_hits() {
        let sample_size: usize = 5;
        let mut registered_hits = RegisteredHits::new(sample_size).unwrap();
        registered_hits.reset_hits();
        assert!(registered_hits.hits.capacity() >= sample_size);
    }
}
