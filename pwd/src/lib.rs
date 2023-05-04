use clap::Parser;
use std::env::current_dir;
use std::path::PathBuf;
use std::io::Error;
use std::fs::canonicalize;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    #[arg(short = 'L', default_value_t=false)]
    default: bool,

    #[arg(short = 'P', default_value_t=false)]
    no_symbolic: bool,
}

pub fn run(config: Cli) -> Result<(), Error> {
    let cur_dir: PathBuf = current_dir()?;

    if config.default || !config.no_symbolic {
        println!("{}", cur_dir.display()); 
    }
    else {
        let sym_dir: PathBuf = canonicalize(cur_dir.as_path())?;
        println!("{}", sym_dir.display());
    }

    Ok(())
}


