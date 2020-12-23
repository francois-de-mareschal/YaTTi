use std::process;
use structopt::StructOpt;
use yatti::Config;

fn main() {
    // Parse provided arguments.
    let options = Config::from_args();

    println!("Hit any key (but q) in cadence (q to quit).");

    // Run the tempo calculator.
    if let Err(e) = yatti::run(options) {
        eprintln!("[ERROR] {}", e);
        process::exit(1);
    }

    println!("Goodbye!");
}
