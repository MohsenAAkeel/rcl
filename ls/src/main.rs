use clap::Parser;
use ls::{run, Cli};

fn main() {
    let config = Cli::parse();
    
    if let Err(e) = run(config){
        panic!("Could not run ls {}", e);
    };
}
