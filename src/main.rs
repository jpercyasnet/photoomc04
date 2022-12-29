extern crate gtk;
extern crate exif;
// extern crate gdk_pixbuf;
extern crate chrono;
extern crate regex;

// use std::fs;
// use std::path::{Path, PathBuf};
// use std::io::BufReader;
// use std::io;
// use std::fs::File;
// use std::process::Command;
use std::env;
// use regex::Regex;

// use dump_file::dump_file;
use build_ui::build_ui;

// use gtk::glib::clone;

// use chrono::prelude::*;
// use chrono::offset::LocalResult;
// use chrono::{Duration, Utc};

// use exif::{Reader, In, Tag};

// use gtk::gdk_pixbuf::{Pixbuf};
// use gtk::glib_sys::ffi::g_main_context_pending;
// use gtk::glib::ffi::GMainContext;
// use gtk::glib::ffi::g_main_context_iteration;
use gtk::prelude::*;
// use gtk::{
//    ProgressBar,
//    Label,
//    Image,
//    FileChooserDialog,
//    FileChooserAction,
//    Button,
//    ComboBoxText,
//    IconView,
//    IconViewExt,
//    Entry,
//    EntryExt,
//    CheckButton,
//    ListStore,
//    TreeModelExt,
//    TreeView,
//    TreeViewColumn,
//    TreeViewExt,
//    CellRendererText,
//    Grid,
//    Notebook,
//    ScrolledWindow,
//    WindowPosition,
//    Window,
//    WindowType,
// };

// const RESPONSE_ACCEPT: i32 = -3;
// const RESPONSE_CANCEL: i32 = -6;
// These two constants stand for the columns of the listmodel and the listview
// const FIRST_COL: i32 = 0;
// const SECOND_COL: i32 = 1;
// const THIRD_COL: i32 = 2;
// const FORTH_COL: i32 = 3;
// const FIFTH_COL: i32 = 4;

    // make moving clones into closures more convenient
//    macro_rules! clone {
//        (@param _) => ( _ );
//        (@param $x:ident) => ( $x );
//        ($($n:ident),+ => move || $body:expr) => (
//            {
//                $( let $n = $n.clone(); )+
//                move || $body
//            }
//        );
//        ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
//            {
//                $( let $n = $n.clone(); )+
//                move |$(clone!(@param $p),)+| $body
//            }
//        );
//    }

// upgrade weak reference or return
//#[macro_export]
//macro_rules! upgrade_weak {
//    ($x:ident, $r:expr) => {{
//        match $x.upgrade() {
//            Some(o) => o,
//            None => return $r,
//        }
//    }};
//    ($x:ident) => {
//        upgrade_weak!($x, ())
 //   };
//}

// function called by Merge to directory button and Merge Merge button
//  Use to get list of sorted files in the Merge to directory in model format
// input is the directory and output is error number, error string and model
mod get_tomodel;

// function called by Organize preview button (twice) and Merge preview button
// use to find the previous file and after file given the current file name
// input is the directory and file name and output is error number, error string, previous filename and after filename
mod get_prevafter;

// function called by Organize directory 1 & 2  buttons and Convert directory button
//  Use to get list of sorted files in the directory list in model format
// input is the directory and output is error number, error string and model
mod get_dirmodel;

// function called by Organize merge button (4 times) and Convert merge button (twice)
//  Use to get list of files (vector list) with there prefix date value, directory number, file name, date substring and orientation
// input is the directory, directory number, suffix file length, date in name boolean, mod year, mod day, mod month,  mod hour, mod min, mod sec
// and output is error number, error string and vector list.
mod get_strvector;

// function called by functions get_tomodel, get_dirmodel, get_strvector (twice)
//  Use to see if file have exif data
// input is the full path filename and output is error number
mod dump_file;

mod parse_moddate;
mod gen_merge;
mod dateinname_merge;
mod celldatename_merge;

mod build_ui;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let application =
        gtk::Application::new(Some("org.jp.photoomc04"), Default::default());

    application.connect_activate(build_ui);

    application.run();

}
