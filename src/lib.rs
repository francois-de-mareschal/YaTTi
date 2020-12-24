use std::collections::VecDeque;
use std::error::Error;
use std::time::Instant;
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
    Ok(())
}

#[derive(Debug, PartialEq)]
struct RegisteredHits {
    hits: VecDeque<Instant>,
    sample_size: u32,
}

impl RegisteredHits {
    fn new(sample_size: u32) -> Result<RegisteredHits, &'static str> {
        // Check the given sample_size is not lower than or equal to one, since it will
        // be impossible to process the duration on zero or one elements.
        if sample_size <= 1 {
            Err("sample size must be at least two.")
        } else {
            let hits: VecDeque<Instant> = VecDeque::with_capacity(sample_size as usize);
            Ok(RegisteredHits { hits, sample_size })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registered_hits_ok() {
        let registered_hits = RegisteredHits::new(10).unwrap();
        assert_eq!(registered_hits.sample_size, 10);
        assert!(registered_hits.hits.capacity() >= 10);
    }

    #[test]
    #[should_panic]
    fn registered_hits_ko_sample_size_le_1() {
        let registered_hits = RegisteredHits::new(1);
        registered_hits.unwrap();
    }
}
