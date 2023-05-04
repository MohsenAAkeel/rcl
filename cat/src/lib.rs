use clap::Parser;
use std::io::{stdin, BufReader, BufRead, Error};
use std::fs::File;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    file_name: Option<String>,

    #[arg(short='E')]
    dollar_terminator: bool,

    #[arg(short='n')]
    number_lines: bool,

}
        

pub fn run(config: Cli) -> Result<(), Error> {
    // open the file or stdin, error if file or stdin can't be opened
    // read n or all lines, prepending line number and/or appending '$'
    // close file/stdout
    
    let ref file_name = match config.file_name {
        Some(ref f) => f,
        None => "-",
    };

    if *file_name == "-" {
        read_from_stdin();
    }
    else {
        if let Err(e) = read_from_file(file_name, &config) {
            panic!("Could not read from file, {}", e);
        };
    }

    Ok(())
}


fn read_from_stdin() -> () {
    loop {
        let mut buffer= String::new();
        stdin().read_line(&mut buffer).expect("Error reading from STDIN");
        print!("{}", buffer);
    }
}

fn read_from_file(file_name: &str, config: &Cli) -> Result<(), Error> {
    let file = File::open(file_name)?;    
    let buf_reader = BufReader::new(file);
    let mut count = 0;

    for line in buf_reader.lines() {
        let mut outline = String::new();
        if config.number_lines {
            outline.push_str(&count.to_string());
        }
        outline.push_str(&line?);
        if config.dollar_terminator {
            outline.push_str("$");
        }
        count += 1;

        println!("{}", outline);
    }


    Ok(())

}
