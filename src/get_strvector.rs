extern crate regex;
extern crate chrono;
extern crate exif;

use regex::Regex;
use chrono::prelude::*;
use chrono::{Duration, Utc};
use chrono::offset::LocalResult;

use exif::{Reader, In, Tag};
use std::fs;
use std::path::{PathBuf};
use std::io::BufReader;
use std::fs::File;
use dump_file::dump_file;

// function called by Organize merge button (4 times) and Convert merge button (twice)
//  Use to get list of files (vector list) with there prefix date value, directory number, file name, date substring and orientation
// input is the directory, directory number, suffix file length, date in name boolean, mod year, mod day, mod month,  mod hour, mod min, mod sec
// and output is error number, error string and vector list.
pub fn get_strvector(current_dir: PathBuf, dirnum: i32, filesize: i32, dateinfile: bool, dateyrx: i64, datedayx: i64, datemox: i64, datehrx: i64, dateminx: i64, datesecx: i64) -> (u32, String, Vec<std::string::String>) {
    let mut errcode: u32 = 0;
    let mut errstring: String = " ".to_string();
    let mut listitems: Vec<String> = Vec::new();
    let mut orient;
    let mut file_prefixdate;
//    let mut dateto = Utc.ymd(2000,1,1).and_hms_milli(1,1,1,0);
    let mut dateyr = 0;
    let mut datemo = 0;
    let mut dateday = 0;
    let mut datehr = 0;
    let mut datemin = 0;
    let mut datesec = 0;
    let mut datenum = 0;
// loop thru directory
    for entry1 in fs::read_dir(&current_dir).unwrap() {
         let entry = entry1.unwrap();
         if let Ok(metadata) = entry.metadata() {
             if let Ok(file_name) = entry.file_name().into_string() {
                 if metadata.is_file() {
//                      if date in name
                     if dateinfile {
                         let file_path = entry.path();
//                         get orientation if available
                         if let Err(_e) = dump_file(&file_path) {
                             orient = format!("0");
                         } else {
                             let file = File::open(file_path).unwrap();
                             let reader = Reader::new().read_from_container(&mut BufReader::new(&file)).unwrap();
                             if let Some(field) = reader.get_field(Tag::Orientation, In::PRIMARY) {
                                 if let Some(width) = field.value.get_uint(0) {
                                     orient = format!("{}", width);
                                 } else {
                                     orient = format!("0");
                                 }
                             } else {
                                 orient = format!("0");
                             }
                         }
// date in name start
                         let date1ar2: Vec<&str> = file_name[0..23].split("_").collect();
                         let lendat2 = date1ar2.len();
                         let mut baddate1 = 0;
                         for indl in 0..lendat2 {
                              let date_int: i32 = date1ar2[indl].clone().parse().unwrap_or(-9999);
                              if date_int == -9999 {
                                  baddate1 = 1;
                                  break;
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
// date in name end
//                          add the mod date values 
                         if baddate1 == 0 {
                             let mut dateto = Utc.ymd(dateyr, datemo, dateday).and_hms_milli(datehr as u32, datemin as u32, datesec as u32, 0);
                             dateto = dateto + Duration::days(dateyrx*365) +
                                                   Duration::days(datemox*30) +
                                                   Duration::days(datedayx) +
                                                   Duration::hours(datehrx) +
                                                   Duration::minutes(dateminx) +
                                                   Duration::seconds(datesecx);
                             file_prefixdate = format!("{}_{:03}", dateto.format("%Y_%m_%d_%H_%M_%S"), datenum);
                         } else {
                             errstring = "<span color=\"#FF000000\">********* get_strvector: BAD DATE  is not correct **********</span>".to_string();
                             errcode = 1;
                             break;
                         }
                     } else {
// if date is not in the file name
                         let mut datetime: DateTime<Local> = metadata.modified().unwrap().into();
                         datetime = datetime + Duration::days(dateyrx*365.0 as i64) +
                                           Duration::days(datemox*30) +
                                           Duration::days(datedayx) +
                                           Duration::hours(datehrx) +
                                           Duration::minutes(dateminx) +
                                           Duration::seconds(datesecx);
                         file_prefixdate = format!("{}_500", datetime.format("%Y_%m_%d_%H_%M_%S"));
//                first set date to the modified date plus mod date
                         let file_path = entry.path();
                         if let Err(_e) = dump_file(&file_path) {
                             orient = format!("0");
                         } else {
//    if exif data first set date to DateTimeOriginal, if not DateTime and add mod date. if any errors, still have modified date
                             let file = File::open(file_path).unwrap();
                             let reader = Reader::new().read_from_container(&mut BufReader::new(&file)).unwrap();
                             if let Some(field) = reader.get_field(Tag::Orientation, In::PRIMARY) {
                                 if let Some(width) = field.value.get_uint(0) {
                                     orient = format!("{}", width);
                                 } else {
                                     orient = format!("0");
                                 }
                             } else {
                                 orient = format!("0");
                             }
                             let mut exifdate = format!(" ");
                             if let Some(field1) = reader.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
                                 exifdate = format!("{}",field1.value.display_as(field1.tag));
                             } else {
                                 if let Some(field1) = reader.get_field(Tag::DateTime, In::PRIMARY) {
                                     exifdate = format!("{}",field1.value.display_as(field1.tag));
                                 }
                             }
                             if exifdate != format!(" ") {
                                 let mut listdatex: Vec<&str> = exifdate[0..10].split("-").collect();
                                 let mut listdatexx: Vec<&str> = exifdate[11..].split(":").collect();
                                 listdatex.append(&mut listdatexx);
                                 let lendatefdx = listdatex.len();
                                 let mut baddatefdx = 0;
                                 for indlfdx in 0..lendatefdx {
                                      let datefdx_int: i32 = listdatex[indlfdx].clone().parse().unwrap_or(-9999);
                                      if datefdx_int == -9999 {
                                          baddatefdx = 1;
                                          break;
                                      } else {
                                          match indlfdx {
                                           0 => dateyr = datefdx_int,
                                           1 => datemo = datefdx_int as u32,
                                           2 => dateday = datefdx_int as u32,
                                           3 => datehr = datefdx_int as i32,
                                           4 => datemin = datefdx_int as i32,
                                           5 => datesec = datefdx_int as i32,
                                           6 => datenum = datefdx_int as i32,
                                           _ => baddatefdx = 1,
                                         }
                                      }
                                 }
                                 if baddatefdx == 0 {
//                                     let mut datetox = Utc.ymd(2000,1,1).and_hms_milli(1,1,1,0);
         
                                     let mut datetox = Utc.ymd(dateyr, datemo, dateday).and_hms_milli(datehr as u32, datemin as u32, datesec as u32, 0);
                                     datetox = datetox + Duration::days(dateyrx*365) +
                                                       Duration::days(datemox*30) +
                                                       Duration::days(datedayx) +
                                                       Duration::hours(datehrx) +
                                                       Duration::minutes(dateminx) +
                                                       Duration::seconds(datesecx);
                                     file_prefixdate = format!("{}_500", datetox.format("%Y_%m_%d_%H_%M_%S"));
                                 } else {
                                     errstring = "<span color=\"#FF000000\">********* get_strvector: BAD DATE  is not correct in exif **********</span>".to_string();
                                     errcode = 2;
                                     break;
                                 }
                             }
                         }
                     }
// process output here
//  call to generate string vector strdate+num | dirnum | fn | newnam | orient
                     let filenamexx = format!("{}", file_name.clone());
                     let strfilesplit: Vec<&str> = filenamexx.split(".").collect();
                     let lenfilesplit = strfilesplit.len();
                     if lenfilesplit != 2 {
                         errstring = "<span color=\"#FF000000\">********* get_strvector: one of filename does not have just 1 period **********</span>".to_string();
                         errcode = 5;
                         break;
                     }
                     let prefix1: String = strfilesplit[0].parse().unwrap();
                     let suffix1: String = strfilesplit[1].parse().unwrap();
                     let mut strlen = prefix1.len() as i32;
                     let mut prefixx: String = "x".to_owned();
                     if strlen < filesize {
                         strlen = strlen + 1;
                         while strlen < filesize {
                                prefixx.push_str("x");
                                strlen = strlen + 1;
                         }
                         prefixx.push_str(&prefix1);
                     } else {
                         prefixx = strfilesplit[0][(strlen - filesize)as usize..].parse().unwrap();
                     }
                     prefixx.push_str(".");
                     prefixx.push_str(&suffix1);
                     let re = Regex::new(r"[^A-Za-z0-9.]").unwrap();
                     let after = re.replace_all(&prefixx, "_");
                     let datesubstrfx = after.to_string();
                     let stroutput = format!("{}|{}|{}|{}|{}", file_prefixdate, dirnum, file_name, datesubstrfx, orient);
                     listitems.push(stroutput);
                 }
             }
         }
    }
    (errcode, errstring, listitems)
}

