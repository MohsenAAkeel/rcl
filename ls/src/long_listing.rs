use std::io;
use std::cmp::Reverse;
use std::time::SystemTime;
use std::os::unix::fs::MetadataExt;
use chrono::{Duration, Local, NaiveDateTime};
use clap::Parser;
use std::{
    io::Error,
    fs::{read_dir, ReadDir, DirEntry, Metadata}
};
use users::{get_user_by_uid, get_group_by_gid};

pub use long_listing;

pub mod long_listing {
    pub fn long_listing_display(contents: Vec<DirEntry>, config: &Cli) -> Result<(), io::Error> {
        //
        let mut output: Vec<Vec<String>> = Vec::new(); 
        
    
        for entry in contents {
            let mut output_line: Vec<String> = Vec::new();
            let metadata = entry.metadata()?;
            
            let mode_string = get_mode_string(&entry, &metadata);
            
            let link_count = metadata.nlink();
    
            let user = get_user_by_uid(metadata.uid()).unwrap();
            let username = user.name();
            
            let username_string = match username.to_str() {
                Some(s) => s,
                _ => panic!("No username")
            };
    
            let group = get_group_by_gid(metadata.gid()).unwrap();
            let groupname = group.name();
    
            let groupname_string = match groupname.to_str() {
                Some(s) => s,
                _ => panic!("No groupname")
            };
    
            let filesize = metadata.len();
            let filename = match entry.file_name().into_string() {
                Ok(s) => s,
                Err(_) => panic!("Could not read unicode")
            };
    
            let ctime = metadata.ctime();
            let offset = Local::now().offset().local_minus_utc();
            let naive_time = match NaiveDateTime::from_timestamp_opt(ctime, 0) {
                Some(s) => s,
                None => panic!("failed")
            };
            let time_adjusted = naive_time + Duration::seconds(i64::from(offset));
            let time_string = time_adjusted.to_string();
    
            output_line.push(mode_string);
            output_line.push(link_count.to_string());
            output_line.push(username_string.to_string());
            output_line.push(groupname_string.to_string());
            output_line.push(filesize.to_string());
            output_line.push(filename);
            output_line.push(time_string);
            output.push(output_line);
        }
    
        Ok(())
    }
}
