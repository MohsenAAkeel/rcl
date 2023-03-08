use std::io;
use std::os::unix::fs::MetadataExt;
use chrono::{Duration, Local, NaiveDateTime};
use std::fs::{DirEntry, Metadata};
use users::{get_user_by_uid, get_group_by_gid};
use crate::Cli;

use crate::utils;
pub use long_listing::long_listing_display;
pub use short_listing::short_listing_display;

mod long_listing {
    use super::*;

    pub fn long_listing_display(contents: Vec<DirEntry>, config: &Cli) -> Result<(), io::Error> {
        /* This function is the entry function to the long listing display
         * option of the ls command. 
         *
         * @params
         *  contents - A vector of DirEntry elements
         *  config - a clap config struct holding all user option input
         *
         * @returns
         *  Returns unit type () 
         */
        // container holding vector of file metadata
        let mut output: Vec<Vec<String>> = Vec::new(); 
        
        // iterate over files and generate relevant listing data    
        for entry in contents {
            // container holding file metadata
            let metadata = entry.metadata()?;
            let output_line: Vec<String> = gather_file_data(&entry, &metadata, config);
            output.push(output_line);
        }
        display_listing(output)
    }

    fn gather_file_data(entry: &DirEntry, metadata: &Metadata, config: &Cli) -> Vec<String> {
            /* this private function collects all relevant file metadata
            * for the long listing display option. 
            *
            * @params
            *   entry - a DirEntry struct for which metadata will be collected
            *   metadata - a Netadata struct containing all relevant file details
            *   config - a clap Cli struct containing user option input
            *
            * @returns
            *   returns a vector of strings, each element corresponding to
            *   a selected piece of file data
            */
            let mut output_line: Vec<String> = Vec::new();
            let mut group: String = "".to_string();
            let mode_string = utils::get_mode_string(&entry, metadata);
            let link_count = metadata.nlink().to_string();
            let user = get_username(metadata);
            if !config.no_group {group = get_group_name(metadata);}
            let filesize = metadata.len();
            let filename = match entry.file_name().into_string() {
                Ok(s) => s,
                Err(_) => panic!("Could not read unicode")
            };
            let file_date = get_file_status_date(metadata);
   
            // pushing to vector in standard order of linux ls 
            // long listing display output
            output_line.push(mode_string);
            output_line.push(link_count);
            output_line.push(user);
            if !config.no_group{output_line.push(group);}
            output_line.push(filesize.to_string());
            output_line.push(file_date);
            output_line.push(filename);

            output_line
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

    fn get_username(metadata: &Metadata) -> String {
        
        let user = get_user_by_uid(metadata.uid()).unwrap();
        let username = user.name();
        
        let username_string = match username.to_str() {
            Some(s) => s,
            _ => "NO USERNAME FOUND"
        };

        username_string.to_string()
    }

    fn get_group_name(metadata: &Metadata) -> String {
        let group = get_group_by_gid(metadata.gid()).unwrap();
        let groupname = group.name();

        let groupname_string = match groupname.to_str() {
            Some(s) => s,
            _ => "NO GROUP ASSIGNED"
        };

        groupname_string.to_string()
    }

    fn get_file_status_date(metadata: &Metadata) -> String {
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


mod short_listing {
    use super::*;
    pub fn short_listing_display(
        contents: Vec<DirEntry>, 
        classify: &bool, 
        fill_width: &bool) -> Result<(), io::Error> {
        // options that can be used here:
        //  almost_all
        //  color
        //  ignore
        //  indicator_style
        
        let mut output: String = "".to_string();
        
        for element in contents {
            let file_name = match element.file_name().into_string() {
                Ok(s) => s,
                Err(_) => panic!("Could not read unicode")
            };
            output.push_str(&file_name);
            if *classify && utils::file_is_dir(&element) {
                output.push_str("/");
            }
            if *fill_width {
                output.push_str(",");
            }
            output.push_str("    ");
        }

        println!("{}", output);

        Ok(())
        
    }
}
