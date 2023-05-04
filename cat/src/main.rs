use clap::Parser;
use cat::{run, Cli};

fn main() {
    let config = Cli::parse();

    if let Err(e) = run(config) {
        panic!("Could not run cat {}", e);
    }
}
