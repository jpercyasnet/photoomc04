extern crate gtk;
extern crate exif;
// extern crate gdk_pixbuf;
extern crate chrono;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use chrono::prelude::*;
use chrono::offset::LocalResult;
use exif::{Reader, In, Tag};
use std::path::{PathBuf};
use gtk::prelude::*;
use gtk::{
    ListStore,
};
use dump_file::dump_file;
const FIRST_COL: i32 = 0;
const SECOND_COL: i32 = 1;

pub fn get_tomodel(cur_dir: PathBuf) -> (u32, String, ListStore) { 
    let errcode: u32;
    let errstring: String;
    let new_model = ListStore::new(&[String::static_type(), String::static_type()]);
    let mut orient;
    let mut numentry = 0;
    let mut baddate1 = 0;
    let mut dateyr = 0;
    let mut datemo = 0;
    let mut dateday = 0;
    let mut datehr = 0;
    let mut datemin = 0;
    let mut datesec = 0;
    let mut datenum = 0;
    let mut listitems: Vec<String> = Vec::new();
// loop thru directory looking for jpg and png files
// these files must have names with prefix yyyy_mm_dd_hh_mm_ss_nnn_
    for entry1 in fs::read_dir(&cur_dir).unwrap() {
         if baddate1 == 0 {
             let entry = entry1.unwrap();
             if let Ok(metadata) = entry.metadata() {
                 if let Ok(file_name) = entry.file_name().into_string() {
                     if metadata.is_file() {
                         if file_name.ends_with(".jpg") | file_name.ends_with(".JPG") |
                            file_name.ends_with(".jpeg") |file_name.ends_with(".JPEG") |
                            file_name.ends_with(".png") |file_name.ends_with(".PNG") {
                             if file_name.len() < 27 {
                                 baddate1 = 1;
                             } else { 
// date from name start
//                                   parse the file name and validate its date
                                 let date1ar2: Vec<&str> = file_name[0..23].split("_").collect();
                                 let lendat2 = date1ar2.len();
                                 for indl in 0..lendat2 {
                                      let date_int: i32 = date1ar2[indl].clone().parse().unwrap_or(-9999);
                                      if date_int == -9999 {
                                          baddate1 = 1;
                                      } else {
                                          match indl {
                                             0 => dateyr = date_int,
                                             1 => datemo = date_int as u32,
                                             2 => dateday = date_int as u32,
                                             3 => datehr = date_int as i32,
                                             4 => datemin = date_int as i32,
                                             5 => datesec = date_int as i32,
                                             6 => datenum = date_int as i32,
                                             _ => baddate1 = 1,
                                          }
                                      }
                                 }
                                 if baddate1 == 0 {
                                     let datexx = Local.ymd_opt(dateyr, datemo, dateday);
                                     if datexx == LocalResult::None {
                                          baddate1 = 1;
                                     } else {
                                         if (datenum < 0) | (datenum > 999) {
                                             baddate1 = 1;
                                         } else if (datehr < 0) | (datehr > 23) {
                                             baddate1 = 1;
                                         } else if (datemin < 0) | (datemin > 59) {
                                             baddate1 = 1;
                                         } else if (datesec < 0) | (datesec > 59) {
                                             baddate1 = 1;
                                         }
                                     }
                                 }
                             }
// date from name end
                             if baddate1 == 0 {
//                       if valid date get the orientation value
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
                                 }
                                 let listival = file_name + "|" + &orient;
//                                 add file name with orientation to list
                                 listitems.push(listival);
                                 numentry = numentry + 1;
                             }
                         } else {
                             baddate1 = 1; // not a jpeg or png file
                         }
                     }
                 } else {
                     baddate1 = 1; // error getting file name
                 }
             } else {
                 baddate1 = 1; // error getting metadata
             }
         }
    } 
// end of for
    if baddate1 == 1 {
        errcode = 2;
        errstring = "<span color=\"#FF000000\">********* get_tomodel: ERROR File format is not correct **********</span>".to_string();
    } else {
        if numentry > 0 {
// sort the list then output to model
            listitems.sort();
            let listitemlen = listitems.len();
            let newtoi = listitemlen as i32 ;
            for indexi in 0..newtoi {
                 let nameplusorient = &listitems[indexi as usize];
                 let namesplit: Vec<&str> = nameplusorient.split("|").collect();
                 new_model.insert_with_values(None,
                        &[FIRST_COL as u32, SECOND_COL as u32],
                        &[&namesplit[0], &namesplit[1]]);
            }
            errcode = 0;                        
            errstring = format!("total to files {}", listitemlen);
        } else {
            errcode = 3;
            errstring = "<span color=\"#FF000000\">********* get_tomodel: directory has no images **********</span>".to_string();
        }
    }
    (errcode, errstring, new_model)
}

