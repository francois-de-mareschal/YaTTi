use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "yatti", about = "Yet another TapTempo implementation.")]
pub struct Config {
    /// Set the precision of the processed tempo.
    #[structopt(short, long, default_value = "0")]
    pub precision: u8,
    /// Set the time (in sec) before the calculation resets to 0.
    #[structopt(short, long, default_value = "5")]
    pub reset_time: u32,
    /// Set the sample size needed to process the tempo.
    #[structopt(short, long, default_value = "5")]
    pub sample_size: u32,
}
