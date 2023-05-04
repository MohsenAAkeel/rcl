use pwd::{run, Cli};
use clap::Parser;

fn main() {
    let config = Cli::parse();

    if let Err(e) = run(config) {
        println!("Error! {}", e);
    }
}

