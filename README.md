# YaTTi is Yet another TapTempo implementation

## TapTempo

TapTempo is a little CLI tool intended to measure a tempo. The original
explanation is 
[here](https://linuxfr.org/users/mzf/journaux/un-tap-tempo-en-ligne-de-commande)
(in French, automatic translation to English
available 
[here](https://translate.google.com/translate?sl=fr&tl=en&u=https://linuxfr.org/users/mzf/journaux/un-tap-tempo-en-ligne-de-commande)); 
long story short, the user hits any key and the software computes 
the corresponding tempo, in Beats Per Minute.

## Build

YaTTi is written in Rust, so once the Rust toolchain is installed, all you
need is ~~love~~:

```sh
cargo test
cargo build --release
```

## Run

Still with ```cargo```, just use

```sh
cargo run --release -- --help
```

to display available options.

## Codestyle

The code is formatted with ```rustfmt``` with all options set to default.

## Contributing

I learnt Rust on my free time during the second lockdown (no more time
wasted in public transports, you nailed it) and this is my very first real
project other than exercices. **If you could be kind enough to give me remarks to the code, it would be really appreciated.**

## Example

Display the help page:

```sh
➜  yatti git:(master) cargo run --release -- --help
    Finished release [optimized] target(s) in 0.03s
     Running `target/release/yatti --help`
yatti 0.3.0
Yet another TapTempo implementation.

USAGE:
    yatti [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --precision <precision>        Set the precision of the processed tempo (max: 5 digits after the decimal point)
                                       [default: 0]
    -r, --reset-time <reset-time>      Set the time (in sec) before the calculation resets to 0 [default: 5]
    -s, --sample-size <sample-size>    Set the sample size needed to process the tempo [default: 5]
```

Run the tool from the git repository:

```sh
➜  yatti git:(master) ✗ cargo run --release          
    Finished release [optimized] target(s) in 0.04s
     Running `target/release/yatti`
Hit any key (but q) in cadence (q to quit).
[INFO] hit any key again to run tempo processing...
[TEMPO] 63 BPM
[TEMPO] 63 BPM
[TEMPO] 62 BPM
[TEMPO] 61 BPM
[TEMPO] 60 BPM
[TEMPO] 70 BPM
[TEMPO] 85 BPM
[TEMPO] 110 BPM
[TEMPO] 153 BPM
[TEMPO] 155 BPM
[TEMPO] 153 BPM
[TEMPO] 156 BPM
[TEMPO] 158 BPM
[TEMPO] 156 BPM
Goodbye!
```

