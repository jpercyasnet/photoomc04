extern crate gtk;
extern crate exif;
extern crate chrono;

// use gtk::prelude::*;
use std::io::BufReader;
use exif::{Reader, In, Tag};
use std::fs;
use chrono::prelude::*;

use std::fs::File;
use std::path::{PathBuf};

use dump_file::dump_file;

use gtk::prelude::*;
use gtk::{
    ListStore,
};

const FIRST_COL: i32 = 0;
const SECOND_COL: i32 = 1;
const THIRD_COL: i32 = 2;
const FORTH_COL: i32 = 3;

// function called by Organize directory 1 & 2  buttons and Convert directory button
//  Use to get list of sorted files in the directory list in model format
// input is the directory and output is error number, error string and model
pub fn get_dirmodel (current_dir: PathBuf) -> (u32, String, ListStore) {
    let errcode: u32;
    let errstring: String;
    let new_modeldir = ListStore::new(&[String::static_type(), String::static_type(), String::static_type(), String::static_type(), String::static_type(), String::static_type()]);
    let mut orient;
//    let mut date_from = format!(" ");
//    let mut file_date = format!(" ");
    let mut listitems: Vec<String> = Vec::new();
    let mut numentry = 0;
    for entry1 in fs::read_dir(&current_dir).unwrap() {
         let entry = entry1.unwrap();
         if let Ok(metadata) = entry.metadata() {
             if let Ok(file_name) = entry.file_name().into_string() {
                 if metadata.is_file() {
                     let datetime: DateTime<Local> = metadata.modified().unwrap().into();
                     let mut file_date = format!("{}", datetime.format("%Y-%m-%d %T"));
                     let mut date_from = format!("file date");
                     let file_path = entry.path();
                     if let Err(e) = dump_file(&file_path) {
                         orient = format!("Meta error : {}", e);
                     } else {
                         let file = File::open(file_path).unwrap();
                         let reader = Reader::new().read_from_container(&mut BufReader::new(&file)).unwrap();
                         if let Some(field) = reader.get_field(Tag::Orientation, In::PRIMARY) {
                             if let Some(width) = field.value.get_uint(0) {
                                 orient = format!("{}", width);
                             } else {
                                 orient = format!("-");
                             }
                         } else {
                             orient = format!("x");
                         }
                         if let Some(field1) = reader.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
                             file_date = format!("{}",field1.value.display_as(field1.tag));
                             date_from = format!("date taken");
                         } else {
                             if let Some(field2) = reader.get_field(Tag::DateTime, In::PRIMARY) {
                                 file_date = format!("{}",field2.value.display_as(field2.tag));
                                 date_from = format!("image date");
                             }
                         }
                     }
                     let listival = file_name + "|" + &date_from + "|" + &file_date + "|" + &orient;
                     listitems.push(listival);
                     numentry = numentry + 1;
                 }
             }
         }
    }
    if numentry > 0 {
        listitems.sort();
        let listitemlen = listitems.len();
        let newtoi = listitemlen as i32 ;
        for indexi in 0..newtoi {
             let namelist = &listitems[indexi as usize];
             let namesplit: Vec<&str> = namelist.split("|").collect();
                 new_modeldir.insert_with_values(None,
                        &[FIRST_COL as u32, SECOND_COL as u32, THIRD_COL as u32, FORTH_COL as u32,],
                        &[&namesplit[0], &namesplit[1], &namesplit[2], &namesplit[3]]);
            }
        errstring = format!("{} files in directory ", numentry);
        errcode = 0;
    } else {
        errstring = "<span color=\"#FF000000\">********* Directory 1: directory has no images **********</span>".to_string();
        errcode = 1;
    }
    (errcode, errstring, new_modeldir)
}

