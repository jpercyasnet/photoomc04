use gtk::gdk_pixbuf::{Pixbuf, Colorspace};

use std::fs;

// function called by Organize preview button (twice) and Merge preview button
// use to find the previous file and after file given the current file name
// input is the directory and file name and output is error number, error string, previous filename and after filename
pub fn get_prevafter(cur_dir: String, tofilename: String, icon_int1: i32) -> (u32, String, String, String, Pixbuf, Pixbuf, Pixbuf) { 
// output addition
    let mut pixbuficonc1: Pixbuf = Pixbuf::new(Colorspace::Rgb,false,8,12,12).unwrap();
    let mut pixbuficonp1: Pixbuf = Pixbuf::new(Colorspace::Rgb,false,8,12,12).unwrap();
    let mut pixbuficona1: Pixbuf = Pixbuf::new(Colorspace::Rgb,false,8,12,12).unwrap();
    
    let mut errcode: u32 = 0;
    let mut errstring: String = " ".to_string();
//    let mut namepo: String = " ".to_string();
//    let mut nameao: String = " ".to_string();
    let mut namec = " ";
    let mut namep = " ";
    let mut namea = " ";
    let mut numentry = 0;
    let mut baddate1 = 0;
    let mut listitems: Vec<String> = Vec::new();
    for entry1 in fs::read_dir(&cur_dir).unwrap() {
         if baddate1 == 0 {
             let entry = entry1.unwrap();
             if let Ok(metadata) = entry.metadata() {
                 if let Ok(file_name) = entry.file_name().into_string() {
                     if metadata.is_file() {
                         if file_name.ends_with(".jpg") | file_name.ends_with(".JPG") |
                            file_name.ends_with(".jpeg") |file_name.ends_with(".JPEG") |
                            file_name.ends_with(".png") |file_name.ends_with(".PNG") {
                             listitems.push(file_name);
                             numentry = numentry + 1;
                         } else {
                             baddate1 = 1;
                         }
                     }
                 } else {
                     baddate1 = 1;
                 }
             } else {
                 baddate1 = 1;
             }
         }
    }
    if baddate1 == 1 {
        errstring = "<span color=\"#FF000000\">********* Preview: ERROR directory does not conform **********</span>".to_string();
        errcode = 1;
    } else {
        if numentry < 1 {
            errstring = "<span color=\"#FF000000\">********* Preview: directory has no images **********</span>".to_string();
            errcode = 2;
        } else {
            listitems.sort();
            let listitemlen = listitems.len();
            let newtoi = listitemlen as i32 ;
            let mut found = 0;
            for indexi in 0..newtoi {
                 if found == 0 {
                     if namep == " " {
                         namep = &listitems[indexi as usize];
                     } else if namec == " " {
                         namec = &listitems[indexi as usize];
                     } else if namea == " " {
                         namea = &listitems[indexi as usize];
                     } else {
                         if tofilename == namep {
                             namea = namec;
                             namec = namep;
                             namep = " ";
                             found = 1;
                         } else if tofilename == namec {
                             found = 1;
                         } else {
                             namep = namec;
                             namec = namea;
                             namea = &listitems[indexi as usize];
                         }
                     }
                 }
            }
            if found == 0 {
                if tofilename == namec {
                    found = 1;
                } else { 
                    if namea != " " {
                        if tofilename == namea {
                            namep = namec;
                            namea = " ";
                            found = 1;
                        }
                    }
                }
            }
            if found == 0 {
                errstring = "<span color=\"#FF000000\">********* Preview: file not found in directory **********</span>".to_string();
                errcode = 3;
            }
        }
    }
    let namepo = namep.to_string();
    let nameao = namea.to_string();
    if errcode == 0 {
        let fullnamec1 = format!("{}/{}", &cur_dir, &tofilename.to_string());
        let pixbufxc1 = Pixbuf::from_file(&fullnamec1).unwrap();
        let mut pixheight1 = pixbufxc1.get_height();
        let mut pixwidth1 = pixbufxc1.get_width();
        if pixheight1 > pixwidth1 {
            pixwidth1 = icon_int1 * pixwidth1 / pixheight1;
            pixheight1 = icon_int1;
        } else {
            pixheight1 = icon_int1 * pixheight1 / pixwidth1;
            pixwidth1 = icon_int1;
        }
        pixbuficonc1 = pixbufxc1.scale_simple(pixwidth1, pixheight1, gtk::gdk_pixbuf::InterpType::Bilinear).unwrap();
//                   ocurrimage1_label.set_from_pixbuf(Some(&pixbuficonc1));
        if namep != " " {
            let fullnamep1 = format!("{}/{}", &cur_dir, &namep);
            let pixbufxp1 = Pixbuf::from_file(&fullnamep1).unwrap();
            pixheight1 = pixbufxp1.get_height();
            pixwidth1 = pixbufxp1.get_width();
            if pixheight1 > pixwidth1 {
                pixwidth1 = icon_int1 * pixwidth1 / pixheight1;
                pixheight1 = icon_int1;
            } else {
                pixheight1 = icon_int1 * pixheight1 / pixwidth1;
                pixwidth1 = icon_int1;
            }
            pixbuficonp1 = pixbufxp1.scale_simple(pixwidth1, pixheight1, gtk::gdk_pixbuf::InterpType::Bilinear).unwrap();
//                                oprevimage1_label.set_from_pixbuf(Some(&pixbuficonp1));
        }
        if namea != " " {
            let fullnamea1 = format!("{}/{}", &cur_dir, &namea);
            let pixbufxa1 = Pixbuf::from_file(&fullnamea1).unwrap();
            pixheight1 = pixbufxa1.get_height();
            pixwidth1 = pixbufxa1.get_width();
            if pixheight1 > pixwidth1 {
                pixwidth1 = icon_int1 * pixwidth1 / pixheight1;
                pixheight1 = icon_int1;
            } else {
                pixheight1 = icon_int1 * pixheight1 / pixwidth1;
                pixwidth1 = icon_int1;
            }
            pixbuficona1 = pixbufxa1.scale_simple(pixwidth1, pixheight1, gtk::gdk_pixbuf::InterpType::Bilinear).unwrap();
//                                oafterimage1_label.set_from_pixbuf(Some(&pixbuficona1));
        }   
    }
    (errcode, errstring, namepo, nameao, pixbuficonc1, pixbuficonp1, pixbuficona1)
}

