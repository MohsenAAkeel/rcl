use std::io;
use std::cmp::Reverse;
use std::time::SystemTime;
use clap::Parser;
use std::{
    io::Error,
    fs::{read_dir, ReadDir, DirEntry}
};

type FileSize = u64;
type FileModTime = SystemTime;
struct FileData{
    file_name: String, 
    mod_time: FileModTime, 
    file_size: FileSize,
    file_type: String,
    permissions: Option<String>,
    group: Option<String>,
    author: Option<String>,
    date: Option<String>
}

impl FileData {
    pub fn new(file_name: String, mod_time: FileModTime, file_size: FileSize, file_type: String) -> FileData {
        FileData {
            file_name,
            mod_time,
            file_size,
            file_type,
            permissions: None,
            group: None,
            author: None,
            date: None,
        }
    }
}

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

    if config.long_listing {
        long_listing_display(config, files)?
    }
    else {
        short_listing_display(config, files)?
    }
    

    Ok(())
}

fn long_listing_display(config: Cli, files: ReadDir) -> Result<(), io::Error> {
    Ok(())
}

fn short_listing_display(config: Cli, files: ReadDir) -> Result<(), io::Error> {
    // options that can be used here:
    //  almost_all
    //  color
    //  ignore
    //  indicator_style
    
    // gather file contents data
    let mut contents: Vec<DirEntry> = gather_dir_entries(files);
  
    //filter dot files
    if !config.all {
        remove_dot_files(&mut contents);
    }   

    if config.ignore_backups {
        remove_backups(&mut contents);
    }

    if config.sort_by_time || config.sort_by_size || config.directory_order {
        sort_files(&config, &mut contents);
    }

    let mut output: String = "".to_string();
    
    for element in contents {
        let file_name = match element.file_name().into_string() {
            Ok(s) => s,
            Err(_) => panic!("Could not read unicode")
        };
        output.push_str(&file_name);
        if config.classify && file_is_dir(&element) {
            output.push_str("/");
        }
        if config.fill_width {
            output.push_str(",");
        }
        output.push_str("    ");
    }

    println!("{}", output);

    Ok(())
    
}

fn gather_dir_entries(files: ReadDir) -> Vec<DirEntry> {
    let contents = files.map(|item| item.unwrap()).collect();
    contents
}

// remove dot files from a vector of FileData objects
fn remove_dot_files(contents: &mut Vec<DirEntry>) -> () {
    contents.retain(|elem| {
        let elem_string = match elem.file_name().into_string() {
            Ok(s) => s,
            Err(_) => panic!("Could not read valid unicode!")
        };
        elem_string.chars().nth(0).unwrap() != '.'
    });
}

// remove backup files from a vector of FileData objects
fn remove_backups(contents: &mut Vec<DirEntry>) {
    contents.retain(|elem| {
        let elem_string = match elem.file_name().into_string() {
            Ok(s) => s,
            Err(_) => panic!("Could not read valid unicode!")
        };
        elem_string.chars().last().unwrap() != '~'
    });
}

fn sort_files(config: &Cli, contents: &mut Vec<DirEntry>) -> () {
    //sorting by time and/or size
    //sorting by both prioritizes time, then size 
    if config.sort_by_time && config.sort_by_size {
       contents.sort_by_key(|elem| Reverse((get_modified_time(elem), get_file_size(elem)))); 
    }
    else if config.sort_by_time {
        contents.sort_by(|a, b| get_modified_time(b).cmp(&get_modified_time(a)));
    }
    else if config.sort_by_size {
        contents.sort_by(|a, b| get_file_size(b).cmp(&get_file_size(a)));
    }

    if config.directory_order {
        contents.sort_by(|a, b| convert_file_type_to_val(b).cmp(&convert_file_type_to_val(a)));
    }
}

fn get_modified_time(elem: &DirEntry) -> SystemTime {
        let elem_metadata = elem.metadata().unwrap();
        
        match elem_metadata.modified() {
            Ok(time) => time,
            Err(_e) => SystemTime::now(),
        }
}

fn get_file_size(elem: &DirEntry) -> FileSize {
    let elem_metadata = elem.metadata().unwrap();
    elem_metadata.len()
}

fn convert_file_type_to_val(elem: &DirEntry) -> i32 {
    let file_type = match elem.file_type() {
        Ok(s) => s,
        Err(_) => panic!("Could not determine file type!")
    };

    if file_type.is_dir() {1}
    else{0}
}

fn file_is_dir(file: &DirEntry) -> bool {
    let file_type = match file.file_type() {
        Ok(s) => s,
        Err(_) => panic!("Could not determine file type!")
    };

    if file_type.is_dir() {true}
    else{false}
}
