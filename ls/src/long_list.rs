use std::io;
use std::os::unix::fs::MetadataExt;
use chrono::{Duration, Local, NaiveDateTime};
use std::fs::DirEntry;
use users::{get_user_by_uid, get_group_by_gid};
use crate::Cli;

use crate::utils;
pub use long_listing::long_listing_display;

pub mod long_listing {
    use super::*;
    pub fn long_listing_display(contents: Vec<DirEntry>, config: &Cli) -> Result<(), io::Error> {
        //
        let mut output: Vec<Vec<String>> = Vec::new(); 
        
    
        for entry in contents {
            let mut output_line: Vec<String> = Vec::new();
            let mut group: String = "".to_string();
            let metadata = entry.metadata()?;
            let mode_string = utils::get_mode_string(&entry, &metadata);
            let link_count = metadata.nlink().to_string();
            let user = get_username(&metadata);
            if !config.no_group {group = get_group_name(&metadata);}
            let filesize = metadata.len();
            let filename = match entry.file_name().into_string() {
                Ok(s) => s,
                Err(_) => panic!("Could not read unicode")
            };
    
            let file_date = get_file_status_date(&metadata);
    
            output_line.push(mode_string);
            output_line.push(link_count);
            output_line.push(user);
            if !config.no_group{output_line.push(group);}
            output_line.push(filesize.to_string());
            output_line.push(file_date);
            output_line.push(filename);
            output.push(output_line);
        }
        display_listing(output)
    }
    
    fn display_listing(output: Vec<Vec<String>>) -> Result<(), io::Error> {
       let mut padding: Vec<usize> = Vec::new();
       
       for i in 0..output.len() {
           for j in 0..output[i].len() {
                if padding.len() < j+1 {
                    padding.push(0);
                }
                let str_length = output[i][j].chars().count();
                if str_length > padding[j] {
                    padding[j] = str_length;
                }
            }
       }

       for i in 0..output.len() {
           let mut output_string: String = "".to_string();
           for j in 0..output[i].len() {
               output_string.push_str(&output[i][j]); 
               output_string.push(' ');
               for _ in 0..(padding[j] - output[i][j].len()) {
                   output_string.push(' ');
                }
            
           }
           println!("{}", output_string);
       }

       Ok(())
    }

    fn get_username(metadata: &dyn MetadataExt) -> String {
        
        let user = get_user_by_uid(metadata.uid()).unwrap();
        let username = user.name();
        
        let username_string = match username.to_str() {
            Some(s) => s,
            _ => "NO USERNAME FOUND"
        };

        username_string.to_string()
    }

    fn get_group_name(metadata: &dyn MetadataExt) -> String {
        let group = get_group_by_gid(metadata.gid()).unwrap();
        let groupname = group.name();

        let groupname_string = match groupname.to_str() {
            Some(s) => s,
            _ => "NO GROUP ASSIGNED"
        };

        groupname_string.to_string()
    }

    fn get_file_status_date(metadata: &dyn MetadataExt) -> String {
        let ctime = metadata.ctime();
        let offset = Local::now().offset().local_minus_utc();
        let naive_time = match NaiveDateTime::from_timestamp_opt(ctime, 0) {
            Some(s) => s,
            None => panic!("Could not get a date of last status change")
        };
        let time_adjusted = naive_time + Duration::seconds(i64::from(offset));

        time_adjusted.to_string()
    }
}
