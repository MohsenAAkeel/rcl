use clap::Parser;
use std::{
    io::Error,
    fs::{read_dir, ReadDir, DirEntry}
};

mod utils;
mod list;
use list::{long_listing_display, short_listing_display};


#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    dir_path: Option<String>,

    #[arg(short, long)]
    all: bool, // include dot files in listing

    #[arg(short='A', long="almost-all")]
    almost_all: bool,  // do not include implied . and .. in listing

    #[arg(long="author")]
    author: bool,  // include the author of each file

    #[arg(short='B', long="ignore-backups")]
    ignore_backups: bool,  // ignore files ending in ~ in listing
    
    #[arg(long)]
    color: Option<String>, // give each file type its own color

    #[arg(short='f')]
    directory_order: bool,  // order files by directory

    #[arg(short='F', long)]
    classify: bool,  // append indicator (*, /, =, >, @, |) to entries 
                     // * - executable
                     // @ - symbolic link
                     // = - socket
                     // | - named pipe
                     // > - door
                     //  / - directory

    #[arg(short='G', long="no-group")]
    no_group: bool,  // in long listing don't include group names

    #[arg(short='r', long="human-readable")]
    human_readable: bool,  // with -l and -s print sizes like 1K, 234M, 2G, etc

    #[arg(short='I', long)]
    ignore: Option<String>,  // do not list entires matching given pattern

    #[arg(short='m')]
    fill_width: bool,  // fill width with comma separated list of entries

    #[arg(short='p', long="indicator-style")]
    indicator_style: bool,  // append slash to directories

    #[arg(short='t')]
    sort_by_time: bool,  // sort entries by time, newest first

    #[arg(short='S')]
    sort_by_size: bool,  // sort entries by size, largest first

    #[arg(short='l')]
    long_listing: bool  //  use long listing format
}

pub fn run(mut config: Cli) -> Result<(), Error> {
    let ref dir = match config.dir_path {
        Some(ref x) => x,
        None => ".",
    };

    let dir_obj = read_dir(dir); 

    let files = match dir_obj {
        Ok(files) => files,
        Err(e) => panic!("ls - Could not read the given file or directory {:?}", e),
    };

    if config.human_readable || config.no_group || config.author {
        config.long_listing = true;
    }
    
    
    let mut contents: Vec<DirEntry> = gather_dir_entries(files);
 
    preprocess_entries(&mut contents, &config);

    if config.long_listing {
        long_listing_display(contents, &config)?
    }
    else {
        short_listing_display(contents, &config.classify, &config.fill_width)?
    }
    

    Ok(())
}

fn preprocess_entries(contents: &mut Vec<DirEntry>, config: &Cli) -> () {
    //filter dot files
    if !config.all {
        utils::remove_dot_files(contents);
    }   

    if config.ignore_backups {
        utils::remove_backups(contents);
    }

    if config.sort_by_time || config.sort_by_size || config.directory_order {
        utils::sort_files(config, contents);
    }
}


fn gather_dir_entries(files: ReadDir) -> Vec<DirEntry> {
    let contents = files.map(|item| item.unwrap()).collect();
    contents
}
