use std::cmp::Reverse;
use crate::Cli;
use std::time::SystemTime;
use std::os::unix::fs::MetadataExt;
use std::fs::{DirEntry, Metadata};

type FileSize = u64;


// remove dot files from a vector of FileData objects
pub fn remove_dot_files(contents: &mut Vec<DirEntry>) -> () {
    contents.retain(|elem| {
        let elem_string = match elem.file_name().into_string() {
            Ok(s) => s,
            Err(_) => panic!("Could not read valid unicode!")
        };
        elem_string.chars().nth(0).unwrap() != '.'
    });
}
    
// remove backup files from a vector of FileData objects
pub fn remove_backups(contents: &mut Vec<DirEntry>) {
    contents.retain(|elem| {
        let elem_string = match elem.file_name().into_string() {
            Ok(s) => s,
            Err(_) => panic!("Could not read valid unicode!")
        };
        elem_string.chars().last().unwrap() != '~'
    });
}

pub fn sort_files(config: &Cli, contents: &mut Vec<DirEntry>) -> () {
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

pub fn get_modified_time(elem: &DirEntry) -> SystemTime {
        let elem_metadata = elem.metadata().unwrap();
        
        match elem_metadata.modified() {
            Ok(time) => time,
            Err(_e) => SystemTime::now(),
        }
}

pub fn get_file_size(elem: &DirEntry) -> FileSize {
    let elem_metadata = elem.metadata().unwrap();
    elem_metadata.len()
}

pub fn convert_file_type_to_val(elem: &DirEntry) -> i32 {
    let file_type = match elem.file_type() {
        Ok(s) => s,
        Err(_) => panic!("Could not determine file type!")
    };

    if file_type.is_dir() {1}
    else{0}
}

pub fn file_is_dir(file: &DirEntry) -> bool {
    let file_type = match file.file_type() {
        Ok(s) => s,
        Err(_) => panic!("Could not determine file type!")
    };

    if file_type.is_dir() {true}
    else{false}
}

pub fn get_mode_string(entry: &DirEntry, metadata: &Metadata) -> String {

    let mode = metadata.mode();
    let mut mode_string: String = "".to_string();

    if file_is_dir(&entry) {mode_string.push('d');}
    else {mode_string.push('-');}

    if mode & 0o400 > 0 {mode_string.push('r');}
    else {mode_string.push('-');}

    if mode & 0o200 > 0 {mode_string.push('w');}
    else {mode_string.push('-');}

    if mode & 0o100 > 0 {mode_string.push('x');}
    else {mode_string.push('-');}

    if mode & 0o40 > 0 {mode_string.push('r');}
    else {mode_string.push('-');}
       
    if mode & 0o20 > 0 {mode_string.push('w');}
    else {mode_string.push('-');}
        
    if mode & 0o10 > 0 {mode_string.push('x');}
    else {mode_string.push('-');}
        
    if mode & 0o4 > 0 {mode_string.push('r');}
    else {mode_string.push('-');} 

    if mode & 0o2 > 0 {mode_string.push('w');}
    else {mode_string.push('-');}

    if mode & 0o1 > 0 {mode_string.push('x');}
    else {mode_string.push('-');}

    mode_string
}

