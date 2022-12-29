extern crate gtk4 as gtk;
extern crate exif;
// extern crate gdk_pixbuf;
extern crate chrono;
extern crate regex;

use gtk::glib;

use std::fs;
use std::path::{PathBuf};
// use std::io::BufReader;
// use std::io;
// use std::fs::File;
use std::process::Command;
// use std::env::args;
use regex::Regex;

// use dump_file::dump_file;
use get_strvector::get_strvector;
use get_dirmodel::get_dirmodel;
use get_prevafter::get_prevafter;
use get_tomodel::get_tomodel;

// use gtk::glib::clone;

use chrono::prelude::*;
use chrono::offset::LocalResult;
use chrono::{Duration, Utc};

// use exif::{Reader, In, Tag};

use gtk::gdk_pixbuf::{Pixbuf};
// use gtk::glib_sys::ffi::g_main_context_pending;
// use gtk::glib::ffi::GMainContext;
// use gtk::glib::ffi::g_main_context_iteration;
use gtk::prelude::*;
use gtk::{
    ProgressBar,
    Label,
    Image,
    FileChooserDialog,
    FileChooserAction,
    Button,
    ComboBoxText,
    IconView,
//    IconViewExt,
    Entry,
//    EntryExt,
    CheckButton,
    ListStore,
    TreeModelExt,
    TreeView,
    TreeViewColumn,
    TreeViewExt,
    CellRendererText,
    Grid,
    Notebook,
    ScrolledWindow,
};

// const RESPONSE_ACCEPT: i32 = -3;
// const RESPONSE_CANCEL: i32 = -6;
// These two constants stand for the columns of the listmodel and the listview
const FIRST_COL: i32 = 0;
const SECOND_COL: i32 = 1;
const THIRD_COL: i32 = 2;
const FORTH_COL: i32 = 3;
const FIFTH_COL: i32 = 4;

pub fn build_ui(application: &gtk::Application) {

//    if gtk::init().is_err() {
//        println!("Failed to initialize GTK.");
//        return;
//    }

//    let window = Window::new();
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("Photo Organize, Merge and Convert Rust GTK4"));
//    window.set_position(WindowPosition::Center);
    window.set_size_request(800, 500);

//    window.connect_delete_event(|win, _| {
//        win.close();
//        gtk::main_quit();
//       Inhibit(false)
//    });

// --------- ORGANIZE TWO DIRECTORIES ---------------

    let odirectory1_button = Button::with_label("In Directory 1");
    let odirectory1_combobox = ComboBoxText::new();
    odirectory1_combobox.set_hexpand(true);

    let otree_view1 = TreeView::new();
    let ocolumn10 = TreeViewColumn::new();
    let ocolumn11 = TreeViewColumn::new();
    let ocolumn12 = TreeViewColumn::new();
    let ocolumn13 = TreeViewColumn::new();
    let ocell10 = CellRendererText::new();
    let ocell11 = CellRendererText::new();
    let ocell12 = CellRendererText::new();
    let ocell13 = CellRendererText::new();
    ocolumn10.pack_start(&ocell10, true);
    ocolumn11.pack_start(&ocell11, true);
    ocolumn12.pack_start(&ocell12, true);
    ocolumn13.pack_start(&ocell13, true);
    // Assiciate view's column with model's id column
    ocolumn10.add_attribute(&ocell10, "text", 0);
    ocolumn11.add_attribute(&ocell11, "text", 1);
    ocolumn12.add_attribute(&ocell12, "text", 2);
    ocolumn13.add_attribute(&ocell13, "text", 3);
    ocolumn10.set_title("Name");
    ocolumn11.set_title("Date From");
    ocolumn12.set_title("Date");
    ocolumn13.set_title("Orient");
    otree_view1.append_column(&ocolumn10);
    otree_view1.append_column(&ocolumn11);
    otree_view1.append_column(&ocolumn12);
    otree_view1.append_column(&ocolumn13);

    let oscroll_window1 = ScrolledWindow::new();
//    oscroll_window1.add(&otree_view1);
    oscroll_window1.set_child(Some(&otree_view1));
    oscroll_window1.set_hexpand(true);
    oscroll_window1.set_vexpand(true);

    let otree_view2 = TreeView::new();
    let ocolumn20 = TreeViewColumn::new();
    let ocolumn21 = TreeViewColumn::new();
    let ocolumn22 = TreeViewColumn::new();
    let ocolumn23 = TreeViewColumn::new();
    let ocell20 = CellRendererText::new();
    let ocell21 = CellRendererText::new();
    let ocell22 = CellRendererText::new();
    let ocell23 = CellRendererText::new();
    ocolumn20.pack_start(&ocell20, true);
    ocolumn21.pack_start(&ocell21, true);
    ocolumn22.pack_start(&ocell22, true);
    ocolumn23.pack_start(&ocell23, true);
    // Assiciate view's column with model's id column
    ocolumn20.add_attribute(&ocell20, "text", 0);
    ocolumn21.add_attribute(&ocell21, "text", 1);
    ocolumn22.add_attribute(&ocell22, "text", 2);
    ocolumn23.add_attribute(&ocell23, "text", 3);
    ocolumn20.set_title("Name");
    ocolumn21.set_title("Date From");
    ocolumn22.set_title("Date");
    ocolumn23.set_title("Orient");
    otree_view2.append_column(&ocolumn20);
    otree_view2.append_column(&ocolumn21);
    otree_view2.append_column(&ocolumn22);
    otree_view2.append_column(&ocolumn23);

    let oscroll_window2 = ScrolledWindow::new();
//    oscroll_window2.add(&otree_view2);
    oscroll_window2.set_child(Some(&otree_view2));
    oscroll_window2.set_hexpand(true);
    oscroll_window2.set_vexpand(true);

    let otree_view3 = TreeView::new();
    let ocolumn30 = TreeViewColumn::new();
    let ocolumn31 = TreeViewColumn::new();
    let ocolumn32 = TreeViewColumn::new();
    let ocolumn33 = TreeViewColumn::new();
    let ocolumn34 = TreeViewColumn::new();
    let ocell30 = CellRendererText::new();
    let ocell31 = CellRendererText::new();
    let ocell32 = CellRendererText::new();
    let ocell33 = CellRendererText::new();
    let ocell34 = CellRendererText::new();
    ocolumn30.pack_start(&ocell30, true);
    ocolumn31.pack_start(&ocell31, true);
    ocolumn32.pack_start(&ocell32, true);
    ocolumn33.pack_start(&ocell33, true);
    ocolumn34.pack_start(&ocell34, true);
    // Assiciate view's column with model's id column
    ocolumn30.add_attribute(&ocell30, "text", 0);
    ocolumn31.add_attribute(&ocell31, "text", 1);
    ocolumn32.add_attribute(&ocell32, "text", 2);
    ocolumn33.add_attribute(&ocell33, "text", 3);
    ocolumn34.add_attribute(&ocell34, "text", 4);
    ocolumn30.set_title("Dir");
    ocolumn31.set_title("Name");
    ocolumn32.set_title("Date-Time-Seq");
    ocolumn33.set_title("NewName");
    ocolumn34.set_title("Orient");
    otree_view3.append_column(&ocolumn30);
    otree_view3.append_column(&ocolumn31);
    otree_view3.append_column(&ocolumn32);
    otree_view3.append_column(&ocolumn33);
    otree_view3.append_column(&ocolumn34);

    let oscroll_window3 = ScrolledWindow::new();
//    oscroll_window3.add(&otree_view3);
    oscroll_window3.set_child(Some(&otree_view3));
    oscroll_window3.set_hexpand(true);
    oscroll_window3.set_vexpand(true);

    let odirectory2_button = Button::with_label("In Directory 2");
    let odirectory2_combobox = ComboBoxText::new();
    odirectory2_combobox.set_hexpand(true);

    let odir1chkbox_check = CheckButton::with_label(Some("Date in Filename 1"));
    let odir2chkbox_check = CheckButton::with_label(Some("Date in Filename 2"));
    let odate1_label = Label::new(Some("-"));
    let odate2_label = Label::new(Some("-"));

    let odatemod1_label = Label::new(Some("date1 mod value (-YY:MM:DD:hh:mm:ss):"));
    let odatemod1_entry = Entry::new();
    odatemod1_entry.set_text("-00:00:00:00:00:00");
    let odatemod2_label = Label::new(Some("date2 mod value (-YY:MM:DD:hh:mm:ss):"));
    let odatemod2_entry = Entry::new();
    odatemod2_entry.set_text("-00:00:00:00:00:00");

    let oprevimage1_label = Image::new();
    let ocurrimage1_label = Image::new();
    let oafterimage1_label = Image::new();
    let oprevimage2_label = Image::new();
    let ocurrimage2_label = Image::new();
    let oafterimage2_label = Image::new();


    let odate_compare_button = Button::with_label("Compare Dates");
    let opreview_button = Button::with_label("Preview Images");
    let oimsize_label = Label::new(Some("size of image:"));
    let oimsize_entry = Entry::new();
    oimsize_entry.set_text("160");

    let ofilesize_label = Label::new(Some("Length of File Description:"));
    let ofilesize_entry = Entry::new();
    ofilesize_entry.set_text("10");
    let odirectory_o_button = Button::with_label("outDirectory");
    let odirectory_o_combobox = ComboBoxText::new();
    odirectory_o_combobox.set_hexpand(true);

    let omerge_button = Button::with_label("Merge");
    let ocopy_button = Button::with_label("Copy");

    let v1box = Grid::new();
    v1box.set_column_spacing(5);
    v1box.set_row_spacing(5);
    v1box.attach(&oprevimage1_label, 0, 3 , 1, 1);
    v1box.attach(&odirectory1_button, 0, 1 , 5, 1);
    v1box.attach(&odirectory1_combobox, 0, 2 , 5, 1);
    v1box.attach(&odirectory2_button, 5, 1 , 5, 1);
    v1box.attach(&odirectory2_combobox, 5, 2 , 5, 1);
    v1box.attach(&oprevimage2_label, 9, 3 , 1, 1);
    v1box.attach(&ocurrimage1_label, 0, 4 , 1, 1);
    v1box.attach(&oscroll_window1, 1, 3 , 4, 3);
    v1box.attach(&oscroll_window2, 5, 3 , 4, 3);
    v1box.attach(&ocurrimage2_label, 9, 4 , 1, 1);
    v1box.attach(&oafterimage1_label, 0, 5 , 1, 1);
    v1box.attach(&oafterimage2_label, 9, 5 , 1, 1);
    v1box.attach(&odir1chkbox_check, 1, 6 , 2, 1);
    v1box.attach(&odate1_label, 3, 6 , 1, 1);
    v1box.attach(&odir2chkbox_check, 5, 6 , 2, 1);
    v1box.attach(&odate2_label, 7, 6 , 1, 1);
    v1box.attach(&odatemod1_label, 0, 7 , 3, 1);
    v1box.attach(&odatemod1_entry, 3, 7 , 1, 1);
    v1box.attach(&odatemod2_label, 5, 7 , 3, 1);
    v1box.attach(&odatemod2_entry, 8, 7 , 1, 1);
    v1box.attach(&odate_compare_button, 1, 8 , 1, 1);
    v1box.attach(&opreview_button, 2, 8 , 1, 1);
    v1box.attach(&oimsize_label, 3, 8 , 1, 1);
    v1box.attach(&oimsize_entry, 4, 8 , 1, 1);
    v1box.attach(&ofilesize_label, 5, 8 , 2, 1);
    v1box.attach(&ofilesize_entry, 7, 8 , 1, 1);
    v1box.attach(&odirectory_o_button, 1, 9 , 2, 1);
    v1box.attach(&odirectory_o_combobox, 3, 9 , 6, 1);
    v1box.attach(&oscroll_window3, 0, 10 , 10, 2); 
    v1box.attach(&omerge_button, 2, 12 , 1, 1); 
    v1box.attach(&ocopy_button, 6, 12 , 1, 1);    

    let vnotebook = Notebook::new();      

    let tab1_label = Label::new(Some("Organize two directories"));
    vnotebook.append_page(&v1box, Some(&tab1_label));

// --- TAB 2 MERGE INTO DIRECTORY ------------------

    let mdirectoryfrom_button = Button::with_label("fromDir");
    let mdirectoryfrom_combobox = ComboBoxText::new();
    mdirectoryfrom_combobox.set_hexpand(true);

    let mdirectoryto_button = Button::with_label("ToDir");
    let mdirectoryto_combobox = ComboBoxText::new();
    mdirectoryto_combobox.set_hexpand(true);

    let mbeforebox_check = CheckButton::with_label(Some("Before (otherwise After)"));

    let mmerge_button = Button::with_label("Merge");

    let mimsize_label = Label::new(Some("size of image:"));
    let mimsize_entry = Entry::new();
    mimsize_entry.set_text("160");

    let miconfrom_view = IconView::new();
    miconfrom_view.set_pixbuf_column(0); // col 0 of the model
    miconfrom_view.set_text_column(1); // col 1 of the model
    miconfrom_view.set_columns(0); // note 6
    miconfrom_view.set_item_width(120); // note 7
    let mscrollfrom_window = ScrolledWindow::new();
//    mscrollfrom_window.add(&miconfrom_view);
    mscrollfrom_window.set_child(Some(&miconfrom_view));
    mscrollfrom_window.set_hexpand(true);
    mscrollfrom_window.set_vexpand(true);

    let mtreeto_view = TreeView::new();
    let mcolumn10 = TreeViewColumn::new();
    let mcolumn11 = TreeViewColumn::new();
    let mcell10 = CellRendererText::new();
    let mcell11 = CellRendererText::new();
    mcolumn10.pack_start(&mcell10, true);
    mcolumn11.pack_start(&mcell11, true);
    // Assiciate view's column with model's id column
    mcolumn10.add_attribute(&mcell10, "text", 0);
    mcolumn11.add_attribute(&mcell11, "text", 1);
    mcolumn10.set_title("Name");
    mcolumn11.set_title("Orientation");
    mtreeto_view.append_column(&mcolumn10);
    mtreeto_view.append_column(&mcolumn11);


    let mscrollto_window = ScrolledWindow::new();
//    mscrollto_window.add(&mtreeto_view);
    mscrollto_window.set_child(Some(&mtreeto_view));
    mscrollto_window.set_hexpand(true);
    mscrollto_window.set_vexpand(true);

//    let previmageto_label = Label::new("previous image");
//    let currimageto_label = Label::new("current image");
//    let afterimageto_label = Label::new("after image");
    let mprevimageto_label = Image::new();
    let mcurrimageto_label = Image::new();
    let mafterimageto_label = Image::new();

    let mpreview_button = Button::with_label("Preview Images");

    let v2box = Grid::new();
    v2box.set_column_spacing(5);
    v2box.set_row_spacing(5);
    v2box.attach(&mdirectoryfrom_button, 0, 1 , 1, 1);
    v2box.attach(&mdirectoryfrom_combobox, 1, 1 , 1, 1);
    v2box.attach(&mdirectoryto_button, 2, 1, 1, 1);
    v2box.attach(&mdirectoryto_combobox, 3, 1, 2, 1);
    v2box.attach(&mbeforebox_check, 3, 2, 1, 1);
    v2box.attach(&mmerge_button, 0, 3, 1, 1); 
    v2box.attach(&mimsize_label, 2, 3 , 1, 1);
    v2box.attach(&mimsize_entry, 3, 3, 1, 1);
    v2box.attach(&mpreview_button, 4, 3 , 1, 1);
    v2box.attach(&mscrollfrom_window, 0, 4 , 2, 4);
    v2box.attach(&mscrollto_window, 2, 4 , 2, 4);
    v2box.attach(&mprevimageto_label, 4, 4 , 1, 1);
    v2box.attach(&mcurrimageto_label, 4, 5 , 1, 1);
    v2box.attach(&mafterimageto_label, 4, 6 , 1, 1);

    let tab2_label = Label::new(Some("Merge into directory"));
    vnotebook.append_page(&v2box, Some(&tab2_label));

// --- TAB 3 CONVERT DIRECTORY ------------------

    let cdirectory1_button = Button::with_label("inDir");
    let cdirectory1_combobox = ComboBoxText::new();
    cdirectory1_combobox.set_hexpand(true);

    let ctree_view1 = TreeView::new();
    let ccolumn10 = TreeViewColumn::new();
    let ccolumn11 = TreeViewColumn::new();
    let ccolumn12 = TreeViewColumn::new();
    let ccolumn13 = TreeViewColumn::new();
    let ccell10 = CellRendererText::new();
    let ccell11 = CellRendererText::new();
    let ccell12 = CellRendererText::new();
    let ccell13 = CellRendererText::new();
    ccolumn10.pack_start(&ccell10, true);
    ccolumn11.pack_start(&ccell11, true);
    ccolumn12.pack_start(&ccell12, true);
    ccolumn13.pack_start(&ccell13, true);
    // Assiciate view's column with model's id column
    ccolumn10.add_attribute(&ccell10, "text", 0);
    ccolumn11.add_attribute(&ccell11, "text", 1);
    ccolumn12.add_attribute(&ccell12, "text", 2);
    ccolumn13.add_attribute(&ccell13, "text", 3);
    ccolumn10.set_title("Name");
    ccolumn11.set_title("Date From");
    ccolumn12.set_title("Date");
    ccolumn13.set_title("Orient");
    ctree_view1.append_column(&ccolumn10);
    ctree_view1.append_column(&ccolumn11);
    ctree_view1.append_column(&ccolumn12);
    ctree_view1.append_column(&ccolumn13);

    let cscroll_window1 = ScrolledWindow::new();
//    cscroll_window1.add(&ctree_view1);
    cscroll_window1.set_child(Some(&ctree_view1));
    cscroll_window1.set_hexpand(true);
    cscroll_window1.set_vexpand(true);

    let ctree_view3 = TreeView::new();
    let ccolumn30 = TreeViewColumn::new();
    let ccolumn31 = TreeViewColumn::new();
    let ccolumn32 = TreeViewColumn::new();
    let ccolumn33 = TreeViewColumn::new();
    let ccolumn34 = TreeViewColumn::new();
    let ccell30 = CellRendererText::new();
    let ccell31 = CellRendererText::new();
    let ccell32 = CellRendererText::new();
    let ccell33 = CellRendererText::new();
    let ccell34 = CellRendererText::new();
    ccolumn30.pack_start(&ccell30, true);
    ccolumn31.pack_start(&ccell31, true);
    ccolumn32.pack_start(&ccell32, true);
    ccolumn33.pack_start(&ccell33, true);
    ccolumn34.pack_start(&ccell34, true);
    // Assiciate view's column with model's id column
    ccolumn30.add_attribute(&ccell30, "text", 0);
    ccolumn31.add_attribute(&ccell31, "text", 1);
    ccolumn32.add_attribute(&ccell32, "text", 2);
    ccolumn33.add_attribute(&ccell33, "text", 3);
    ccolumn34.add_attribute(&ccell34, "text", 4);
    ccolumn30.set_title("Dir");
    ccolumn31.set_title("Name");
    ccolumn32.set_title("Date-Time-Seq");
    ccolumn33.set_title("NewName");
    ccolumn34.set_title("Orient");
    ctree_view3.append_column(&ccolumn30);
    ctree_view3.append_column(&ccolumn31);
    ctree_view3.append_column(&ccolumn32);
    ctree_view3.append_column(&ccolumn33);
    ctree_view3.append_column(&ccolumn34);

    let cscroll_window3 = ScrolledWindow::new();
//    cscroll_window3.add(&ctree_view3);
    cscroll_window3.set_child(Some(&ctree_view3));
    cscroll_window3.set_hexpand(true);
    cscroll_window3.set_vexpand(true);

    let cdatemod1_label = Label::new(Some("date mod value (-YY:MM:DD:hh:mm:ss):"));
    let cdatemod1_entry = Entry::new();
    cdatemod1_entry.set_text("-00:00:00:00:00:00");

    let cfilesize_label = Label::new(Some("Length of File Description:"));
    let cfilesize_entry = Entry::new();
    cfilesize_entry.set_text("10");
    let cdirectory_o_button = Button::with_label("outDirectory");
    let cdirectory_o_combobox = ComboBoxText::new();
    cdirectory_o_combobox.set_hexpand(true);

    let cmerge_button = Button::with_label("Merge");
    let ccopy_button = Button::with_label("Copy");

    let v3box = Grid::new();
    v3box.set_column_spacing(5);
    v3box.set_row_spacing(5);
    v3box.attach(&cdirectory1_button, 0, 1 , 1, 1);
    v3box.attach(&cdirectory1_combobox, 1, 1 , 3, 1);
    v3box.attach(&cscroll_window1, 0, 2 , 4, 2);
    v3box.attach(&cdatemod1_label, 0, 5 , 2, 1);
    v3box.attach(&cdatemod1_entry, 2, 5 , 1, 1);
    v3box.attach(&cfilesize_label, 5, 5 , 1, 1);
    v3box.attach(&cfilesize_entry, 6, 5 , 1, 1);
    v3box.attach(&cdirectory_o_button, 0, 7 , 1, 1);
    v3box.attach(&cdirectory_o_combobox, 1, 7 , 2, 1);
    v3box.attach(&cscroll_window3, 0, 8 , 10, 2); 
    v3box.attach(&cmerge_button, 2, 10 , 1, 1); 
    v3box.attach(&ccopy_button, 6, 10 , 1, 1);    

    let tab3_label = Label::new(Some("Convert one directory"));
    vnotebook.append_page(&v3box, Some(&tab3_label));

    let messagetitle_label = Label::new(Some("Message: "));
    let messageval_label = Label::new(Some("Message area"));

    let progress_progressbar = ProgressBar::new();
    progress_progressbar.set_show_text(true);

    let vbox = Grid::new();

    vbox.set_column_spacing(5);
    vbox.set_row_spacing(5);

    vbox.attach(&messagetitle_label, 0, 0 , 1, 1);
    vbox.attach(&messageval_label, 1, 0 , 9, 1);
    vbox.attach(&vnotebook, 0, 2, 10, 10);
    vbox.attach(&progress_progressbar, 0, 13 , 10, 1);

//    window.add(&vbox);
    window.set_child(Some(&vbox));
    window.set_destroy_with_parent(true);
    window.show(); 

//----------------- org directory 1 button start -----------------------------------
    odirectory1_button.connect_clicked(glib::clone!(@weak window, @weak odirectory1_combobox, @weak messageval_label, @weak otree_view1 =>  move|_| {

        messageval_label.set_text("getting directory 1");

        let dialog = FileChooserDialog::new(
            Some("Choose a Directory 1"),
            Some(&window),
            FileChooserAction::SelectFolder,
            &[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
        );

        dialog.connect_response(move |d: &FileChooserDialog, response: gtk::ResponseType| {
            if response == gtk::ResponseType::Ok {
                if let Some(foldername) = d.get_file() {
                    if let Some(folderpath) = foldername.get_path() {
                        odirectory1_combobox.prepend_text(&folderpath.display().to_string());
                        odirectory1_combobox.set_active(Some(0));
                        let current_dir = folderpath;
                        let (errcd, errstr, newmodel) = get_dirmodel(current_dir);
                        if errcd == 0 {
                            otree_view1.set_model(Some(&newmodel)); 
                            messageval_label.set_text(&errstr);
                        } else {
                            messageval_label.set_markup(&errstr);
                        }
                    } else { 
                        messageval_label.set_markup("<span color=\"#FF000000\">********* Directory 1: ERROR GETTING PATH **********</span>");
                    }
                } else { 
                    messageval_label.set_markup("<span color=\"#FF000000\">********* Directory 1: ERROR GETTING FILE **********</span>");
                }
            }
            if messageval_label.get_text() == "getting directory 1" {
                messageval_label.set_markup("<span color=\"#FF000000\">********* Directory 1: ERROR  OPEN  button not selected **********</span>");
            }
            d.close();
        });
        dialog.show();
    }));
//----------------- org directory 1 button end -----------------------------------

//----------------- org directory 2 button start -----------------------------------
    odirectory2_button.connect_clicked(glib::clone!(@weak window, @weak odirectory2_combobox, @weak messageval_label, @weak otree_view2 => move|_| {
    
        messageval_label.set_text("getting directory 2");

        let dialog = FileChooserDialog::new(
            Some("Choose a Directory 2"),
            Some(&window.clone()),
            FileChooserAction::SelectFolder,
            &[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
        );

        dialog.connect_response(move |d: &FileChooserDialog, response: gtk::ResponseType| {
            if response == gtk::ResponseType::Ok {
                if let Some(foldername) = d.get_file() {
                    if let Some(folderpath) = foldername.get_path() {
                        odirectory2_combobox.prepend_text(&folderpath.display().to_string());
                        odirectory2_combobox.set_active(Some(0));
                        let current_dir = folderpath;
                        let (errcd, errstr, newmodel) = get_dirmodel(current_dir);
                        if errcd == 0 {
                            otree_view2.set_model(Some(&newmodel));
                            messageval_label.set_text(&errstr);
                        } else {
                            messageval_label.set_markup(&errstr);
                        }
                    } else { 
                        messageval_label.set_markup("<span color=\"#FF000000\">********* Directory 2: ERROR GETTING PATH **********</span>");
                    }
                } else { 
                    messageval_label.set_markup("<span color=\"#FF000000\">********* Directory 2: ERROR GETTING FILE **********</span>");
                }
            }
            if messageval_label.get_text() == "getting directory 2" {
                messageval_label.set_markup("<span color=\"#FF000000\">********* Directory 2: ERROR  OPEN  button not selected **********</span>");
            }
            d.close();
        });
        dialog.show();
    }));
//----------------- org directory 2 button end -----------------------------------

//----------------- org out directory button start -----------------------------------
    odirectory_o_button.connect_clicked(glib::clone!(@weak window, @weak odirectory_o_combobox, @weak messageval_label => move|_| {
    
        messageval_label.set_text("getting directory out");
        
        let dialog = FileChooserDialog::new(
            Some("Choose a Directory"),
            Some(&window.clone()),
            FileChooserAction::SelectFolder,
            &[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
        );

        dialog.connect_response(move |d: &FileChooserDialog, response: gtk::ResponseType| {
            if response == gtk::ResponseType::Ok {
                if let Some(foldername) = d.get_file() {
                    if let Some(folderpath) = foldername.get_path() {
                        odirectory_o_combobox.prepend_text(&folderpath.display().to_string());
                        odirectory_o_combobox.set_active(Some(0));
                        messageval_label.set_text("org out directory selected");
                    } else { 
                        messageval_label.set_markup("<span color=\"#FF000000\">********* org Out Directory: ERROR GETTING PATH **********</span>");
                    }
                } else { 
                    messageval_label.set_markup("<span color=\"#FF000000\">********* org Out Directory: ERROR GETTING FILE **********</span>");
                }
            }
            if messageval_label.get_text() == "getting directory out" {
                messageval_label.set_markup("<span color=\"#FF000000\">********* org Out Directory: ERROR  OPEN  button not selected **********</span>");
            }
            d.close();
        });
        dialog.show();
    }));
//----------------- org out directory button end -----------------------------------

//----------------- org preview button start -----------------------------------
//    let odirectory1_combobox_weakop = odirectory1_combobox.downgrade();
//    let otree_view1_weakop = otree_view1.downgrade();
//    let oprevimage1_label_weakop = oprevimage1_label.downgrade();
//    let ocurrimage1_label_weakop = ocurrimage1_label.downgrade();
//    let oafterimage1_label_weakop = oafterimage1_label.downgrade();
//    let odirectory2_combobox_weakop = odirectory2_combobox.downgrade();
//    let otree_view2_weakop = otree_view2.downgrade();
//    let oprevimage2_label_weakop = oprevimage2_label.downgrade();
//    let ocurrimage2_label_weakop = ocurrimage2_label.downgrade();
//    let oafterimage2_label_weakop = oafterimage2_label.downgrade();
//    let messageval_label_weakop = messageval_label.downgrade();
//    let oimsize_entry_weakop = oimsize_entry.downgrade();

    let odirectory1_combobox_cloneop = odirectory1_combobox.clone();
    let otree_view1_cloneop = otree_view1.clone();
    let oprevimage1_label_cloneop = oprevimage1_label.clone();
    let ocurrimage1_label_cloneop = ocurrimage1_label.clone();
    let oafterimage1_label_cloneop = oafterimage1_label.clone();
    let odirectory2_combobox_cloneop = odirectory2_combobox.clone();
    let otree_view2_cloneop = otree_view2.clone();
    let oprevimage2_label_cloneop = oprevimage2_label.clone();
    let ocurrimage2_label_cloneop = ocurrimage2_label.clone();
    let oafterimage2_label_cloneop = oafterimage2_label.clone();
    let messageval_label_cloneop = messageval_label.clone();
    let oimsize_entry_cloneop = oimsize_entry.clone();
    opreview_button.connect_clicked( move|_| {
//    opreview_button.connect_clicked(clone!(odirectory1_combobox_weakop, otree_view1_weakop, oprevimage1_label_weakop, ocurrimage1_label_weakop, oafterimage1_label_weakop, odirectory2_combobox_weakop, otree_view2_weakop, oprevimage2_label_weakop, ocurrimage2_label_weakop, oafterimage2_label_weakop, messageval_label_weakop, oimsize_entry_weakop  => move|_| {
//        let odirectory1_combobox = upgrade_weak!(odirectory1_combobox_weakop);
//        let otree_view1 = upgrade_weak!(otree_view1_weakop);
//        let oprevimage1_label = upgrade_weak!(oprevimage1_label_weakop);
//        let ocurrimage1_label = upgrade_weak!(ocurrimage1_label_weakop);
//        let oafterimage1_label = upgrade_weak!(oafterimage1_label_weakop);
//        let odirectory2_combobox = upgrade_weak!(odirectory2_combobox_weakop);
//        let otree_view2 = upgrade_weak!(otree_view2_weakop);
//        let oprevimage2_label = upgrade_weak!(oprevimage2_label_weakop);
//        let ocurrimage2_label = upgrade_weak!(ocurrimage2_label_weakop);
//        let oafterimage2_label = upgrade_weak!(oafterimage2_label_weakop);
//        let messageval_label = upgrade_weak!(messageval_label_weakop);
//        let oimsize_entry = upgrade_weak!(oimsize_entry_weakop);

        let mut badsize_int = 1;
        let mut icon_int1 = 0;

        let inputic_text = oimsize_entry_cloneop.get_text();
        let icon_int: i32 = inputic_text.parse().unwrap_or(-99);
        if icon_int > 0 {
            badsize_int = 0;
            icon_int1 = icon_int;
        } else if icon_int == -99 {
            messageval_label_cloneop.set_markup("<span color=\"#FF000000\">********* org Preview: Icon Size is not an integer **********</span>");
        } else {
            messageval_label_cloneop.set_markup("<span color=\"#FF000000\">********* org Preview: Icon Size not positive integer **********</span>");
        }
        if badsize_int == 0 {
            if (icon_int1 < 50) | (icon_int1 > 255) {
                messageval_label_cloneop.set_markup("<span color=\"#FF000000\">********* org Preview: Icon Size not between 50 and 255 **********</span>");
                badsize_int = 1;
            }
        }
        if badsize_int == 0 {
            if let Some(cur_dir1) = odirectory1_combobox_cloneop.get_active_text() {
                let str_cur_dir1 = &cur_dir1;
                let treemodel1 = otree_view1_cloneop.get_model();
                if treemodel1 == None {
                    messageval_label_cloneop.set_markup("<span color=\"#FF000000\">********* org Preview: ERROR NOTHING IN DIRECTORY 1 LIST **********</span>");
                    badsize_int = 1;
                } else {
                    let selectiont1 = otree_view1_cloneop.get_selection();
                    if let Some((modelt1, itert1)) = selectiont1.get_selected() {
                        let tofilenameval1 = modelt1.get_value(&itert1, 0).get::<String>();
                        let tofilenamestr1 = format!("{:?}", tofilenameval1);
                        let tofileln1 = tofilenamestr1.len();
                        let tofileend1 = tofileln1 - 3;
                        let tofilestart1 = 9;
                        let tofilename1: String = tofilenamestr1.get(tofilestart1..tofileend1).unwrap().to_string();
                        let tofilenamex1: String = tofilenamestr1.get(tofilestart1..tofileend1).unwrap().to_string();
                        let tofilenamey1: String = tofilenamestr1.get(tofilestart1..tofileend1).unwrap().to_string();
                        let (errcd1, errstr1, namep1, namea1) = get_prevafter(str_cur_dir1.to_string(), tofilename1);
                        if errcd1 > 0 {
                            messageval_label_cloneop.set_markup(&errstr1);
                            badsize_int = 1;
                        } else {
                            let fullnamec1 = format!("{}/{}", &str_cur_dir1, &tofilenamex1.to_string());
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
                            let pixbuficonc1: Pixbuf = pixbufxc1.scale_simple(pixwidth1, pixheight1, gtk::gdk_pixbuf::InterpType::Bilinear).unwrap();
                            ocurrimage1_label_cloneop.set_from_pixbuf(Some(&pixbuficonc1));
                            if namep1 == " " {
                                oprevimage1_label_cloneop.clear();
                            } else {
                                let fullnamep1 = format!("{}/{}", &str_cur_dir1, &namep1);
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
                                let pixbuficonp1: Pixbuf = pixbufxp1.scale_simple(pixwidth1, pixheight1, gtk::gdk_pixbuf::InterpType::Bilinear).unwrap();
                                oprevimage1_label_cloneop.set_from_pixbuf(Some(&pixbuficonp1));
                            }
                            if namea1 == " " {
                                oafterimage1_label_cloneop.clear();
                            } else {
                                let fullnamea1 = format!("{}/{}", &str_cur_dir1, &namea1);
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
                                let pixbuficona1: Pixbuf = pixbufxa1.scale_simple(pixwidth1, pixheight1, gtk::gdk_pixbuf::InterpType::Bilinear).unwrap();
                                oafterimage1_label_cloneop.set_from_pixbuf(Some(&pixbuficona1));
                            }
                            let msgstr = format!("org Previewed: {} -- {} -- {}", namep1, tofilenamey1, namea1);
                            messageval_label_cloneop.set_text(&msgstr);
                        }
                    } else {
                        messageval_label_cloneop.set_markup("<span color=\"#FF000000\">********* org Preview: NO SELECTION IN DIRECTORY 1 **********</span>");
                        badsize_int = 1;
                    }            
                }
            } else {
                messageval_label_cloneop.set_markup("<span color=\"#FF000000\">********* org Preview: ERROR GETTING TO DIRECTORY 1 COMBOBOX **********</span>");
                badsize_int = 1;
            }
        }
        if badsize_int == 0 {
            if let Some(cur_dir2) = odirectory2_combobox_cloneop.get_active_text() {
                let str_cur_dir2 = &cur_dir2;
                let treemodel2 = otree_view2_cloneop.get_model();
                if treemodel2 == None {
                    messageval_label_cloneop.set_markup("<span color=\"#FF000000\">********* org Preview: ERROR NOTHING IN DIRECTORY 2 LIST **********</span>");
                } else {
                    let selectiont2 = otree_view2_cloneop.get_selection();
                    if let Some((modelt2, itert2)) = selectiont2.get_selected() {
                        let tofilenameval2 = modelt2.get_value(&itert2, 0).get::<String>();
                        let tofilenamestr2 = format!("{:?}", tofilenameval2);
                        let tofileln2 = tofilenamestr2.len();
                        let tofileend2 = tofileln2 - 3;
                        let tofilestart2 = 9;
                        let tofilename2: String = tofilenamestr2.get(tofilestart2..tofileend2).unwrap().to_string();
                        let tofilenamex2: String = tofilenamestr2.get(tofilestart2..tofileend2).unwrap().to_string();
                        let tofilenamey2: String = tofilenamestr2.get(tofilestart2..tofileend2).unwrap().to_string();
                        let (errcd2, errstr2, namep2, namea2) = get_prevafter(str_cur_dir2.to_string(), tofilename2);
                        if errcd2 > 0 {
                            messageval_label_cloneop.set_markup(&errstr2);
                        } else {
                            let fullnamec2 = format!("{}/{}", &str_cur_dir2, &tofilenamex2.to_string());
                            let pixbufxc2 = Pixbuf::from_file(&fullnamec2).unwrap();
                            let mut pixheight2 = pixbufxc2.get_height();
                            let mut pixwidth2 = pixbufxc2.get_width();
                            if pixheight2 > pixwidth2 {
                                pixwidth2 = icon_int1 * pixwidth2 / pixheight2;
                                pixheight2 = icon_int1;
                            } else {
                                pixheight2 = icon_int1 * pixheight2 / pixwidth2;
                                pixwidth2 = icon_int1;
                            }
                            let pixbuficonc2: Pixbuf = pixbufxc2.scale_simple(pixwidth2, pixheight2, gtk::gdk_pixbuf::InterpType::Bilinear).unwrap();
                            ocurrimage2_label_cloneop.set_from_pixbuf(Some(&pixbuficonc2));
                            if namep2 == " " {
                                oprevimage2_label_cloneop.clear();
                            } else {
                                let fullnamep2 = format!("{}/{}", &str_cur_dir2, &namep2);
                                let pixbufxp2 = Pixbuf::from_file(&fullnamep2).unwrap();
                                pixheight2 = pixbufxp2.get_height();
                                pixwidth2 = pixbufxp2.get_width();
                                if pixheight2 > pixwidth2 {
                                    pixwidth2 = icon_int1 * pixwidth2 / pixheight2;
                                    pixheight2 = icon_int1;
                                } else {
                                    pixheight2 = icon_int1 * pixheight2 / pixwidth2;
                                    pixwidth2 = icon_int1;
                                }
                                let pixbuficonp2: Pixbuf = pixbufxp2.scale_simple(pixwidth2, pixheight2, gtk::gdk_pixbuf::InterpType::Bilinear).unwrap();
                                oprevimage2_label_cloneop.set_from_pixbuf(Some(&pixbuficonp2));
                            }
                            if namea2 == " " {
                                oafterimage2_label_cloneop.clear();
                            } else {
                                let fullnamea2 = format!("{}/{}", &str_cur_dir2, &namea2);
                                let pixbufxa2 = Pixbuf::from_file(&fullnamea2).unwrap();
                                pixheight2 = pixbufxa2.get_height();
                                pixwidth2 = pixbufxa2.get_width();
                                if pixheight2 > pixwidth2 {
                                    pixwidth2 = icon_int1 * pixwidth2 / pixheight2;
                                    pixheight2 = icon_int1;
                                } else {
                                    pixheight2 = icon_int1 * pixheight2 / pixwidth2;
                                    pixwidth2 = icon_int1;
                                }
                                let pixbuficona2: Pixbuf = pixbufxa2.scale_simple(pixwidth2, pixheight2, gtk::gdk_pixbuf::InterpType::Bilinear).unwrap();
                                oafterimage2_label_cloneop.set_from_pixbuf(Some(&pixbuficona2));
                            }
                            let msgstr = format!("org Previewed: {} -- {} -- {}", namep2, tofilenamey2, namea2);
                            messageval_label_cloneop.set_text(&msgstr);
                        }
                    } else {
                        messageval_label_cloneop.set_markup("<span color=\"#FF000000\">********* org Preview: NO SELECTION IN DIRECTORY 1 **********</span>");
                    }            
                }
            } else {
                messageval_label_cloneop.set_markup("<span color=\"#FF000000\">********* org Preview: ERROR GETTING TO DIRECTORY 1 COMBOBOX **********</span>");
            }
        }
    });

//----------------- org preview button end -----------------------------------

//----------------- org merge button start -----------------------------------
    
//    let odirectory1_combobox_weakom = odirectory1_combobox.downgrade();
//    let otree_view1_weakom = otree_view1.downgrade();
//    let odirectory2_combobox_weakom = odirectory2_combobox.downgrade();
//    let otree_view2_weakom = otree_view2.downgrade();
//    let otree_view3_weakom = otree_view3.downgrade();
//    let ofilesize_entry_weakom = ofilesize_entry.downgrade();
//    let odatemod1_entry_weakom = odatemod1_entry.downgrade();
//    let odatemod2_entry_weakom = odatemod2_entry.downgrade();
//    let odir1chkbox_check_weakom = odir1chkbox_check.downgrade();
//    let odir2chkbox_check_weakom = odir2chkbox_check.downgrade();
//    let messageval_label_weakom = messageval_label.downgrade();

//    let odirectory1_combobox_cloneom = odirectory1_combobox.clone();
//    let otree_view1_cloneom = otree_view1.clone();
//    let odirectory2_combobox_cloneom = odirectory2_combobox.clone();
//    let otree_view2_cloneom = otree_view2.clone();
//    let otree_view3_cloneom = otree_view3.clone();
//    let ofilesize_entry_cloneom = ofilesize_entry.clone();
//    let odatemod1_entry_cloneom = odatemod1_entry.clone();
//    let odatemod2_entry_cloneom = odatemod2_entry.clone();
//    let odir1chkbox_check_cloneom = odir1chkbox_check.clone();
//    let odir2chkbox_check_cloneom = odir2chkbox_check.clone();
//    let messageval_label_cloneom = messageval_label.clone();
//    omerge_button.connect_clicked(move|_| {

    omerge_button.connect_clicked(glib::clone!(@weak odirectory1_combobox, @weak otree_view1, @weak odirectory2_combobox, @weak otree_view2, @weak otree_view3, @weak ofilesize_entry, @weak odatemod1_entry, @weak odatemod2_entry, @weak odir1chkbox_check, @weak odir2chkbox_check, @weak messageval_label  => move|_| {
//    omerge_button.connect_clicked(clone!(odirectory1_combobox_weakom, otree_view1_weakom, odirectory2_combobox_weakom, otree_view2_weakom, otree_view3_weakom, ofilesize_entry_weakom, odatemod1_entry_weakom, odatemod2_entry_weakom, odir1chkbox_check_weakom, odir2chkbox_check_weakom, messageval_label_weakom  => move|_| {
//        let odirectory1_combobox = upgrade_weak!(odirectory1_combobox_weakom);
//        let otree_view1 = upgrade_weak!(otree_view1_weakom);
//        let odirectory2_combobox = upgrade_weak!(odirectory2_combobox_weakom);
//        let otree_view2 = upgrade_weak!(otree_view2_weakom);
//        let otree_view3 = upgrade_weak!(otree_view3_weakom);
        let mut filesize_int: i32 = 0;
//        let ofilesize_entry = upgrade_weak!(ofilesize_entry_weakom);
//        let odatemod1_entry = upgrade_weak!(odatemod1_entry_weakom);
//        let odatemod2_entry = upgrade_weak!(odatemod2_entry_weakom);
//        let odir1chkbox_check = upgrade_weak!(odir1chkbox_check_weakom);
//        let odir2chkbox_check = upgrade_weak!(odir2chkbox_check_weakom);
//        let messageval_label = upgrade_weak!(messageval_label_weakom);

        let mut str_cur_dir1 = format!(" ");
        let mut str_cur_dir2 = format!(" ");
        let mut dateyr1 = 0;
        let mut datemo1 = 0;
        let mut dateday1 = 0;
        let mut datehr1 = 0;
        let mut datemin1 = 0;
        let mut datesec1 = 0;
        let mut dateyr2 = 0;
        let mut datemo2 = 0;
        let mut dateday2 = 0;
        let mut datehr2 = 0;
        let mut datemin2 = 0;
        let mut datesec2 = 0;

        let mut bolok = true;

// see if directories have files
        let treemodel1 = otree_view1.get_model();
        if treemodel1 == None {
             messageval_label.set_markup("<span color=\"#FF000000\">********* org Merge: ERROR NOTHING IN DIRECTORY 1 LIST **********</span>");
             bolok = false;
        }
        if bolok {
            let treemodel2 = otree_view2.get_model();
            if treemodel2 == None {
                messageval_label.set_markup("<span color=\"#FF000000\">********* org Merge: ERROR NOTHING IN DIRECTORY 2 LIST **********</span>");
                bolok = false;
            }
        }

// make sure both directories exist and are not the same
        if bolok {
            if let Some(cur_dir1) = odirectory1_combobox.get_active_text() {
                str_cur_dir1 = format!("{}", cur_dir1);
                if let Some(cur_dir2) = odirectory2_combobox.get_active_text() {
                    str_cur_dir2 = format!("{}", cur_dir2);
                    if str_cur_dir1 == str_cur_dir2 {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* org Merge: DIR 1 AND DIR 2 ARE THE SAME DIRECTORY **********</span>");
                        bolok = false;
                    }
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* org Merge: ERROR GETTING DIRECTORY 2 IN COMBOBOX **********</span>");
                    bolok = false;
                }
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* org Merge: ERROR GETTING DIRECTORY 1 IN COMBOBOX **********</span>");
                bolok = false;
            }
        }
// see if filesize exists and is between 4 and 16
        if bolok {
            let filesize_text = ofilesize_entry.get_text();
            filesize_int = filesize_text.parse().unwrap_or(-99);
            if filesize_int > 0 {
                if (filesize_int < 4) | (filesize_int > 16) {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* org Merge: Invalid file length. Must be between 4 and 16 **********</span>");
                    bolok = false;
                }
            } else if filesize_int == -99 {
                messageval_label.set_markup("<span color=\"#FF000000\">********* org Merge: Files length is not an integer **********</span>");
                bolok = false;
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* org Merge: File length is not positive integer **********</span>");
                bolok = false;
            }
        }
// validate date mod 1 & 2
        if bolok {
            let datemod1_text = odatemod1_entry.get_text();
            let mut baddate1 = 0;
            let datemod1_textx: &String = &format!("{}", datemod1_text);
            let date1ar1: Vec<&str> = datemod1_textx[0..].split(":").collect();
            let lendat1 = date1ar1.len();
            if (lendat1 > 6) | (lendat1 < 6) {
                baddate1 = 1;
            } else {
                for indl in 0..lendat1 {
                     let date_int: i32 = date1ar1[indl].clone().parse().unwrap_or(-9999);
                     if date_int == -9999 {
                         baddate1 = 1;
                     } else {
                         match indl {
                            0 => dateyr1 = date_int as i64,
                            1 => datemo1 = date_int as i64,
                            2 => dateday1 = date_int as i64,
                            3 => datehr1 = date_int as i64,
                            4 => datemin1 = date_int as i64,
                            5 => datesec1 = date_int as i64,
                            _ => baddate1 = 1,
                         }
                     }
                }
            }
            if baddate1 == 1 {
                messageval_label.set_markup("<span color=\"#FF000000\">********* org Merge: Date Mod 1 is not formatted correctly **********</span>");
                bolok = false;
            }
        }
        if bolok {
            let datemod2_text = odatemod2_entry.get_text();
            let mut baddate2 = 0;
            let datemod2_textx: &String = &format!("{}", datemod2_text);
            let date1ar2: Vec<&str> = datemod2_textx[0..].split(":").collect();
            let lendat2 = date1ar2.len();
            if (lendat2 > 6) | (lendat2 < 6) {
                baddate2 = 1;
            } else {
                for indl in 0..lendat2 {
                     let date_int: i32 = date1ar2[indl].clone().parse().unwrap_or(-9999);
                     if date_int == -9999 {
                         baddate2 = 1;
                     } else {
                         match indl {
                            0 => dateyr2 = date_int as i64,
                            1 => datemo2 = date_int as i64,
                            2 => dateday2 = date_int as i64,
                            3 => datehr2 = date_int as i64,
                            4 => datemin2 = date_int as i64,
                            5 => datesec2 = date_int as i64,
                            _ => baddate2 = 1,
                         }
                     }
                }
            }
            if baddate2 == 1 {
                messageval_label.set_markup("<span color=\"#FF000000\">********* org Merge: Date Mod 2 is not formatted correctly **********</span>");
                bolok = false;
            }
        }
        if bolok {
            let current_dir = PathBuf::from(&str_cur_dir1);
            let (errcd1, errstr1, newvector1) = get_strvector(current_dir, 1, filesize_int, odir1chkbox_check.get_active(), dateyr1, dateday1, datemo1, datehr1, datemin1, datesec1);
            let mut newvectormut = newvector1;
            let mut chgseq2 = false;
            if errcd1 != 0 {
                messageval_label.set_markup(&errstr1);
                bolok = false;
            } else {
                let current_dir2 = PathBuf::from(&str_cur_dir2);
                let (errcd2, errstr2, newvector2) = get_strvector(current_dir2, 2, filesize_int, odir2chkbox_check.get_active(), dateyr2, dateday2, datemo2, datehr2, datemin2, datesec2);
                if errcd2 != 0 {
                    messageval_label.set_markup(&errstr2);
                    bolok = false;
                 } else {
                    let mut newvectormut2 = newvector2;
                    newvectormut.append(&mut newvectormut2);
                 }
            }
            if bolok {
                let newvectormutlen = newvectormut.len();
                let newtoi = newvectormutlen as i32 ;
                if newtoi < 2 {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* org Merge: Only one entry in both directories **********</span>");
                    bolok = false;
                } else {
                    newvectormut.sort();
                    let mut chgx = true;
                    while chgx {
                           let mut listitems: Vec<String> = Vec::new();
                           chgx = false;
                           for indexi in 1..newtoi {
                                let strinput1split: Vec<&str> = newvectormut[(indexi - 1) as usize].split("|").collect();
                                let strinputsplit: Vec<&str> = newvectormut[indexi as usize].split("|").collect();
                                let mut file_prefixdate1 = format!(" ");
                                let mut file_prefixdate2 = format!(" ");
                                if chgseq2 {
                                    chgseq2 = false; 
                                    let prefix1: String = strinput1split[0][0..19].parse().unwrap();
                                    let mut seq2_int: i32 = strinput1split[0][20..].parse().unwrap_or(-9999);
                                    if seq2_int == -9999 {
                                        bolok = false;
                                        chgx = false;
                                        messageval_label.set_markup("<span color=\"#FF000000\">********* org Merge: seq number not numeric **********</span>");
                                        break;
                                    } else {
                                        if seq2_int < 999 {
                                            seq2_int = seq2_int + 1;
                                            chgx = true;
                                        }
                                        file_prefixdate1 = format!("{}_{:03}", prefix1, seq2_int);
                                    }
                                } else {
                                    file_prefixdate1 = format!("{}", strinput1split[0]);
                                }
                                file_prefixdate2 = format!("{}", strinputsplit[0]);
                                if file_prefixdate1 == file_prefixdate2 {
                                    chgseq2 = true;
                                    let prefix1: String = strinput1split[0][0..19].parse().unwrap();
                                    let mut seq1_int: i32 = strinput1split[0][20..].parse().unwrap_or(-9999);
                                    if seq1_int == -9999 {
                                        bolok = false;
                                        chgx = false;
                                        messageval_label.set_markup("<span color=\"#FF000000\">********* org Merge: seq number not numeric **********</span>");
                                        break;
                                    } else {
                                        if seq1_int > 0 {
                                            seq1_int = seq1_int - 1;
                                            chgx = true;
                                        }
                                    }    
                                    file_prefixdate1 = format!("{}_{:03}", prefix1, seq1_int);
                                }
                                let stroutput = format!("{}|{}|{}|{}|{}", file_prefixdate1, strinput1split[1], strinput1split[2], strinput1split[3], strinput1split[4]);
                                listitems.push(stroutput);
                           } // end for 
                           if bolok {
                               let current_dira = PathBuf::from(&str_cur_dir1);
                               let (errcda, errstra, newvectora) = get_strvector(current_dira, 1, filesize_int, odir1chkbox_check.get_active(), dateyr1, dateday1, datemo1, datehr1, datemin1, datesec1);
                               let mut newvectormuta = newvectora;
                               if errcda != 0 {
                                   messageval_label.set_markup(&errstra);
                                   bolok = false;
                                   chgx = false;
                                   break;
                               } else {
                                   let current_dirb = PathBuf::from(&str_cur_dir2);
                                   let (errcdb, errstrb, newvectorb) = get_strvector(current_dirb, 2, filesize_int, odir2chkbox_check.get_active(), dateyr2, dateday2, datemo2, datehr2, datemin2, datesec2);
                                   if errcdb != 0 {
                                       messageval_label.set_markup(&errstrb);
                                       bolok = false;
                                       chgx = false;
                                       break;
                                   } else {
                                       let mut newvectormutb = newvectorb;
                                       newvectormuta.append(&mut newvectormutb);
                                       newvectormuta.sort();
                                       let strinputxsplit: Vec<&str> = newvectormuta[(newtoi - 1) as usize].split("|").collect();
                                       let mut file_prefixdatex = format!(" ");
                                       if chgseq2 {
                                           chgseq2 = false; 
                                           let prefixx: String = strinputxsplit[0][0..19].parse().unwrap();
                                           let mut seqx_int: i32 = strinputxsplit[0][20..].parse().unwrap_or(-9999);
                                           if seqx_int == -9999 {
                                               bolok = false;
                                               chgx = false;
                                               messageval_label.set_markup("<span color=\"#FF000000\">********* org Merge: seq number not numeric **********</span>");
                                               break;
                                           } else {
                                               if seqx_int < 999 {
                                                   seqx_int = seqx_int + 1;
                                                   chgx = true;
                                               }
                                               file_prefixdatex = format!("{}_{:03}", prefixx, seqx_int);
                                           }
                                       } else {
                                           file_prefixdatex = format!("{}", strinputxsplit[0]);
                                       }
                                       let stroutputx = format!("{}|{}|{}|{}|{}", file_prefixdatex, strinputxsplit[1], strinputxsplit[2], strinputxsplit[3], strinputxsplit[4]);
                                       listitems.push(stroutputx);
                                       newvectormut = listitems;
                                       newvectormut.sort();
                                   }
                               }
                           }
                    }  // end of while
                } 
            }
            if bolok {
                let newvectormutlen = newvectormut.len();
                let newtoi = newvectormutlen as i32 ;
                let new_model = ListStore::new(&[String::static_type(), String::static_type(), String::static_type(), String::static_type(), String::static_type(), String::static_type()]);
                for indexi in 0..newtoi {
                     let strinputx = &newvectormut[indexi as usize];
                     let strinputsplitx: Vec<&str>  = strinputx.split("|").collect();
                     new_model.insert_with_values(None,
                                 &[FIRST_COL as u32, SECOND_COL as u32, THIRD_COL as u32, FORTH_COL as u32, FIFTH_COL as u32,],
                                 &[&strinputsplitx[1], &strinputsplitx[2], &strinputsplitx[0], &strinputsplitx[3], &strinputsplitx[4]]);
                }
                otree_view3.set_model(Some(&new_model));
                let messvalx = format!("organize merge merged {} files", newtoi);
                messageval_label.set_text(&messvalx);
           }
        }
    }));
//----------------- org merge button end -----------------------------------

//----------------- org copy button start -----------------------------------
//    let odirectory1_combobox_weakoc = odirectory1_combobox.downgrade();
//    let odirectory2_combobox_weakoc = odirectory2_combobox.downgrade();
//    let odirectory_o_combobox_weakoc = odirectory_o_combobox.downgrade();
//    let otree_view3_weakoc = otree_view3.downgrade();
//    let messageval_label_weakoc = messageval_label.downgrade();
//    let progress_progressbar_weakoc = progress_progressbar.downgrade();

//    let odirectory1_combobox_cloneoc = odirectory1_combobox.clone();
//    let odirectory2_combobox_cloneoc = odirectory2_combobox.clone();
//    let odirectory_o_combobox_cloneoc = odirectory_o_combobox.clone();
//    let otree_view3_cloneoc = otree_view3.clone();
//    let messageval_label_cloneoc = messageval_label.clone();
//    let progress_progressbar_cloneoc = progress_progressbar.clone();
//    ocopy_button.connect_clicked(move|_| { 

    ocopy_button.connect_clicked(glib::clone!(@weak odirectory1_combobox, @weak odirectory2_combobox, @weak otree_view3, @weak progress_progressbar, @weak messageval_label  => move|_| {

//    ocopy_button.connect_clicked(clone!(odirectory1_combobox_weakoc, odirectory2_combobox_weakoc, otree_view3_weakoc, progress_progressbar_weakoc, messageval_label_weakoc  => move|_| {
//        let odirectory1_combobox = upgrade_weak!(odirectory1_combobox_weakoc);
//        let odirectory2_combobox = upgrade_weak!(odirectory2_combobox_weakoc);
//        let odirectory_o_combobox = upgrade_weak!(odirectory_o_combobox_weakoc);
//        let otree_view3 = upgrade_weak!(otree_view3_weakoc);
//        let progress_progressbar = upgrade_weak!(progress_progressbar_weakoc);
//        let messageval_label = upgrade_weak!(messageval_label_weakoc);

        let mut bolok = true;
        let mut str_cur_dir1 = format!(" ");
        let mut str_cur_dir2 = format!(" ");
        let mut str_cur_dir_o = format!(" ");
        let mut str_cur_dirfrom = format!(" ");

// check if both directories exist and they are not equal
        if bolok {
            if let Some(cur_dir1) = odirectory1_combobox.get_active_text() {
                str_cur_dir1 = cur_dir1.to_string();
                if let Some(cur_dir2) = odirectory2_combobox.get_active_text() {
                    str_cur_dir2 = cur_dir2.to_string();
                    if str_cur_dir1 == str_cur_dir2 {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* org COPY: DIR 1 AND DIR 2 ARE THE SAME DIRECTORY **********</span>");
                        bolok = false;
                    }
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* org COPY: ERROR GETTING TO DIRECTORY IN COMBOBOX 2 **********</span>");
                    bolok = false;
                }
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* org COPY: ERROR GETTING FROM DIRECTORY IN COMBOBOX 1 **********</span>");
                bolok = false;
            }
        }

// check if outdirectory has files (must not have files)
        if bolok {
            if let Some(cur_dir_o) = odirectory_o_combobox.get_active_text() {
                str_cur_dir_o = cur_dir_o.to_string();
                for entry1 in fs::read_dir(&str_cur_dir_o).unwrap() {
                     let entry = entry1.unwrap();
                     if let Ok(metadata) = entry.metadata() {
                         if let Ok(_file_name) = entry.file_name().into_string() {
                             if metadata.is_file() {
                                 messageval_label.set_markup("<span color=\"#FF000000\">********* org COPY: OUTPUT DIRECTORY HAS FILES IN IT **********</span>");
                                 bolok = false;
                             }
                         }
                     }
                }
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* org COPY: ERROR GETTING OUT DIRECTORY IN COMBOBOX  **********</span>");
                bolok = false;
           }
        }
// check if merge files and if so process
        if bolok {
            let view3model = otree_view3.get_model();
            if view3model == None {
                messageval_label.set_markup("<span color=\"#FF000000\">********* org Copy: ERROR NOTHING IN MERGE LIST **********</span>");
                bolok = false;
            } else {
                progress_progressbar.set_fraction(0.0);
//                while gtk::events_pending() {
//                    gtk::main_iteration_do(true);
//                }
                let view3modeluw = view3model.unwrap();
                let mut valid = true;
                let validval = view3modeluw.get_iter_first().unwrap();
                let mut numrow = 0;
                let numchildren = view3modeluw.iter_n_children(None);
                let mut numprocess = 0;
                while valid {
                      let dirval = view3modeluw.get_value(&validval,0).get::<String>();
                      let strdirval = format!("{:?}", dirval);
                      let dirln = strdirval.len();
                      let dirend = dirln - 3;
                      let dirstart = 9;
                      let dirx = strdirval.get(dirstart..dirend);
                      let dir_int: i32 = dirx.unwrap().parse().unwrap_or(-99);
                      if dir_int == 1 {
                          str_cur_dirfrom = str_cur_dir1.clone();
                      } else if dir_int == 2 {
                          str_cur_dirfrom = str_cur_dir2.clone();
                      } else {
                          messageval_label.set_markup("<span color=\"#FF000000\">********* org COPY: invalid directory number in list **********</span>");
                          break;
                      }
                      let filefromval = view3modeluw.get_value(&validval,1).get::<String>();
                      let filefromstr = format!("{:?}", filefromval);
                      let filefromln = filefromstr.len();
                      let filefromend = filefromln - 3;
                      let filefromstart = 9;
                      let filefromx = filefromstr.get(filefromstart..filefromend).unwrap();
                      let fullfrom = str_cur_dirfrom.clone() + "/" + filefromx;


                      let filepreval = view3modeluw.get_value(&validval,2).get::<String>();
                      let fileprestr = format!("{:?}", filepreval);
                      let filepreln = fileprestr.len();
                      let filepreend = filepreln - 3;
                      let fileprestart = 9;
                      let fileprex = fileprestr.get(fileprestart..filepreend).unwrap();


                      let filetoval = view3modeluw.get_value(&validval,3).get::<String>();
                      let filetostr = format!("{:?}", filetoval);
                      let filetoln = filetostr.len();
                      let filetoend = filetoln - 3;
                      let filetostart = 9;
                      let filetox = filetostr.get(filetostart..filetoend).unwrap();

                      let fullto = str_cur_dir_o.clone() + "/" + fileprex + "_" + filetox;
                      valid = view3modeluw.iter_next(&validval);
//                      println!("copy action of: cp -p {} {}", fullfrom, fullto);
                      if valid & (numprocess < 4) {
                          Command::new("cp")
                                  .arg("-p")
                                  .arg(&fullfrom)
                                  .arg(&fullto)
                                  .spawn()
                                  .expect("failed to execute process");
                          numprocess = numprocess + 1;
                      } else {
                          let _output = Command::new("cp")
                                                .arg("-p")
                                                .arg(&fullfrom)
                                                .arg(&fullto)
                                                .output()
                                                .expect("failed to execute process");
                          numprocess = 0;
                     }
                     numrow = numrow + 1;
                     let progressfr: f64 = numrow as f64 / numchildren as f64;
                     progress_progressbar.set_fraction(progressfr);
//                     while gtk::events_pending() {
//                          gtk::main_iteration_do(true);
//                     }
                }
                let messvalx = format!("organize copy copied {} files", numchildren);
                messageval_label.set_text(&messvalx);
            }
        }
    }));

//----------------- org copy button end -----------------------------------

//----------------- org date compare button start -----------------------------------
    
//    let odirectory1_combobox_weakor = odirectory1_combobox.downgrade();
//    let otree_view1_weakor = otree_view1.downgrade();
//    let odirectory2_combobox_weakor = odirectory2_combobox.downgrade();
//    let otree_view2_weakor = otree_view2.downgrade();
//    let odatemod1_entry_weakor = odatemod1_entry.downgrade();
//    let odatemod2_entry_weakor = odatemod2_entry.downgrade();
//    let odir1chkbox_check_weakor = odir1chkbox_check.downgrade();
//    let odir2chkbox_check_weakor = odir2chkbox_check.downgrade();
//    let odate1_label_weakor = odate1_label.downgrade();
//    let odate2_label_weakor = odate2_label.downgrade();
//    let messageval_label_weakor = messageval_label.downgrade();

//    let odirectory1_combobox_cloneor = odirectory1_combobox.clone();
//    let otree_view1_cloneor = otree_view1.clone();
//    let odirectory2_combobox_cloneor = odirectory2_combobox.clone();
//    let otree_view2_cloneor = otree_view2.clone();
//    let odatemod1_entry_cloneor = odatemod1_entry.clone();
//    let odatemod2_entry_cloneor = odatemod2_entry.clone();
//    let odir1chkbox_check_cloneor = odir1chkbox_check.clone();
//    let odir2chkbox_check_cloneor = odir2chkbox_check.clone();
//    let odate1_label_cloneor = odate1_label.clone();
//    let odate2_label_cloneor = odate2_label.clone();
//    let messageval_label_cloneor = messageval_label.clone();
//    odate_compare_button.connect_clicked( move|_| {

    odate_compare_button.connect_clicked(glib::clone!(@weak odirectory1_combobox, @weak otree_view1, @weak odirectory2_combobox, @weak otree_view2, @weak odatemod1_entry, @weak odatemod2_entry, @weak odir1chkbox_check, @weak odir2chkbox_check, @weak odate1_label, @weak odate2_label, @weak messageval_label  => move|_| {

//    odate_compare_button.connect_clicked(clone!(odirectory1_combobox_weakor, otree_view1_weakor, odirectory2_combobox_weakor, otree_view2_weakor, odatemod1_entry_weakor, odatemod2_entry_weakor, odir1chkbox_check_weakor, odir2chkbox_check_weakor, odate1_label_weakor, odate2_label_weakor, messageval_label_weakor  => move|_| {
//        let odirectory1_combobox = upgrade_weak!(odirectory1_combobox_weakor);
//        let otree_view1 = upgrade_weak!(otree_view1_weakor);
//        let odirectory2_combobox = upgrade_weak!(odirectory2_combobox_weakor);
//        let otree_view2 = upgrade_weak!(otree_view2_weakor);
//        let odatemod1_entry = upgrade_weak!(odatemod1_entry_weakor);
//        let odatemod2_entry = upgrade_weak!(odatemod2_entry_weakor);
//        let odir1chkbox_check = upgrade_weak!(odir1chkbox_check_weakor);
//        let odir2chkbox_check = upgrade_weak!(odir2chkbox_check_weakor);
//        let odate1_label = upgrade_weak!(odate1_label_weakor);
//        let odate2_label = upgrade_weak!(odate2_label_weakor);
//        let messageval_label = upgrade_weak!(messageval_label_weakor);

        let mut tofilename1 = format!(" ");
        let mut tofilename2 = format!(" ");
        let mut filedate1 = format!(" ");
        let mut filedate2 = format!(" ");
        let mut file_formdate1 = format!(" ");
        let mut file_formdate2 = format!(" ");
        let mut dateto = Utc.ymd(2000,1,1).and_hms_milli(1,1,1,0);
        let mut dateyr1 = 0;
        let mut datemo1 = 0;
        let mut dateday1 = 0;
        let mut datehr1 = 0;
        let mut datemin1 = 0;
        let mut datesec1 = 0;
        let mut dateyr2 = 0;
        let mut datemo2 = 0;
        let mut dateday2 = 0;
        let mut datehr2 = 0;
        let mut datemin2 = 0;
        let mut datesec2 = 0;
        let mut dateyr = 0;
        let mut datemo = 0;
        let mut dateday = 0;
        let mut datehr = 0;
        let mut datemin = 0;
        let mut datesec = 0;
        let mut datenum = 0;
        let mut dateyr11 = 0;
        let mut datemo11 = 0;
        let mut dateday11 = 0;
        let mut datehr11 = 0;
        let mut datemin11 = 0;
        let mut datesec11 = 0;
        let mut dateyr21 = 0;
        let mut datemo21 = 0;
        let mut dateday21 = 0;
        let mut datehr21 = 0;
        let mut datemin21 = 0;
        let mut datesec21 = 0;
        let mut dateyr111 = 0;
        let mut datemo111 = 0;
        let mut dateday111 = 0;
        let mut datehr111 = 0;
        let mut datemin111 = 0;
        let mut datesec111 = 0;
        let mut dateyr222 = 0;
        let mut datemo222 = 0;
        let mut dateday222 = 0;
        let mut datehr222 = 0;
        let mut datemin222 = 0;
        let mut datesec222 = 0;

        let mut bolok = true;

        if let Some(_cur_dir1) = odirectory1_combobox.get_active_text() {
            let treemodel1 = otree_view1.get_model();
            if treemodel1 == None {
                messageval_label.set_markup("<span color=\"#FF000000\">********* org compare: ERROR NOTHING IN DIRECTORY 1 LIST **********</span>");
                bolok = false;
            } else {
                let selectiont1 = otree_view1.get_selection();
                if let Some((modelt1, itert1)) = selectiont1.get_selected() {
                    let tofilenameval1 = modelt1.get_value(&itert1, 0).get::<String>();
                    let tofilenamestr1 = format!("{:?}", tofilenameval1);
                    let tofileln1 = tofilenamestr1.len();
                    let tofileend1 = tofileln1 - 3;
                    let tofilestart1 = 9;
                    tofilename1 = tofilenamestr1.get(tofilestart1..tofileend1).unwrap().to_string();
                    if !odir1chkbox_check.get_active() {
                        let takedate1 = modelt1.get_value(&itert1, 1).get::<String>();
                        let takedatestr1 = format!("{:?}", takedate1);
                        if takedatestr1.len() < 31 {
                            let imgdate1 = modelt1.get_value(&itert1, 2).get::<String>();
                            let imgdatestr1 = format!("{:?}", imgdate1);
                            if imgdatestr1.len() < 31 {
                                let moddate1 = modelt1.get_value(&itert1, 3).get::<String>();
                                let moddatestr1 = format!("{:?}", moddate1);
                                if moddatestr1.len() < 31 {
                                    messageval_label.set_markup("<span color=\"#FF000000\">********* org compare: invalid date in selection 1 **********</span>");
                                    bolok = false;
                                } else {
                                    filedate1 = moddatestr1;
                                }
                            } else {
                                filedate1 = imgdatestr1;
                            }
                        } else {
                            filedate1 = takedatestr1;
                        }                               
                    }
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* org compare: NO SELECTION IN DIRECTORY 1 **********</span>");
                    bolok = false;
                }            
            }
        } else {
            messageval_label.set_markup("<span color=\"#FF000000\">********* org compare: ERROR GETTING TO DIRECTORY 1 COMBOBOX **********</span>");
            bolok = false;
        }
      
        if bolok {
            if let Some(_cur_dir2) = odirectory2_combobox.get_active_text() {
                let treemodel2 = otree_view2.get_model();
                if treemodel2 == None {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* org compare: ERROR NOTHING IN DIRECTORY 2 LIST **********</span>");
                    bolok = false;
                } else {
                    let selectiont2 = otree_view2.get_selection();
                    if let Some((modelt2, itert2)) = selectiont2.get_selected() {
                        let tofilenameval2 = modelt2.get_value(&itert2, 0).get::<String>();
                        let tofilenamestr2 = format!("{:?}", tofilenameval2);
                        let tofileln2 = tofilenamestr2.len();
                        let tofileend2 = tofileln2 - 3;
                        let tofilestart2 = 9;
                        tofilename2 = tofilenamestr2.get(tofilestart2..tofileend2).unwrap().to_string();
                        if !odir2chkbox_check.get_active() {
                            let takedate2 = modelt2.get_value(&itert2, 1).get::<String>();
                            let takedatestr2 = format!("{:?}", takedate2);
                            if takedatestr2.len() < 31 {
                                let imgdate2 = modelt2.get_value(&itert2, 2).get::<String>();
                                let imgdatestr2 = format!("{:?}", imgdate2);
                                if imgdatestr2.len() < 31 {
                                    let moddate2 = modelt2.get_value(&itert2, 3).get::<String>();
                                    let moddatestr2 = format!("{:?}", moddate2);
                                    if moddatestr2.len() < 31 {
                                        messageval_label.set_markup("<span color=\"#FF000000\">********* org compare: invalid date in selection 2 **********</span>");
                                        bolok = false;
                                    } else {
                                        filedate2 = moddatestr2;
                                    }
                                } else {
                                    filedate2 = imgdatestr2;
                                }
                            } else {
                                filedate2 = takedatestr2;
                            }                               
                        }
                    } else {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* org compare: NO SELECTION IN DIRECTORY 2 **********</span>");
                        bolok = false;
                    }            
                }
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* org compare: ERROR GETTING TO DIRECTORY 2 COMBOBOX **********</span>");
                bolok = false;
            }
        }
// validate date mod 1 & 2
        if bolok {
            let datemod1_text = odatemod1_entry.get_text();
            let datemod1_textx: &String = &format!("{}", datemod1_text);
            let date1ar1: Vec<&str> = datemod1_textx[0..].split(":").collect();
            let lendat1 = date1ar1.len();
            if (lendat1 > 6) | (lendat1 < 6) {
                bolok = false;
            } else {
                for indl in 0..lendat1 {
                     let date_int: i32 = date1ar1[indl].clone().parse().unwrap_or(-9999);
                     if date_int == -9999 {
                         bolok = false;
                     } else {
                         match indl {
                            0 => dateyr1 = date_int as i64,
                            1 => datemo1 = date_int as i64,
                            2 => dateday1 = date_int as i64,
                            3 => datehr1 = date_int as i64,
                            4 => datemin1 = date_int as i64,
                            5 => datesec1 = date_int as i64,
                            _ => bolok = false,
                         }
                     }
                }
            }
            if !bolok {
                messageval_label.set_markup("<span color=\"#FF000000\">********* org compare: Date Mod 1 is not formatted correctly **********</span>");
            }
        }
        if bolok {
            let datemod2_text = odatemod2_entry.get_text();
            let datemod2_textx: &String = &format!("{}", datemod2_text);
            let date1ar2: Vec<&str> = datemod2_textx[0..].split(":").collect();
            let lendat2 = date1ar2.len();
            if (lendat2 > 6) | (lendat2 < 6) {
                bolok = false;
            } else {
                for indl in 0..lendat2 {
                     let date_int: i32 = date1ar2[indl].clone().parse().unwrap_or(-9999);
                     if date_int == -9999 {
                         bolok = false;
                     } else {
                         match indl {
                            0 => dateyr2 = date_int as i64,
                            1 => datemo2 = date_int as i64,
                            2 => dateday2 = date_int as i64,
                            3 => datehr2 = date_int as i64,
                            4 => datemin2 = date_int as i64,
                            5 => datesec2 = date_int as i64,
                            _ => bolok = false,
                         }
                     }
                }
            }
            if !bolok {
                messageval_label.set_markup("<span color=\"#FF000000\">********* org compar: Date Mod 2 is not formatted correctly **********</span>");
            }
        }
        if bolok {

            if odir1chkbox_check.get_active() {
                let xfilename: &String = &format!("{}", tofilename1);
// date from name start
                let date1ar2: Vec<&str> = xfilename[0..23].split("_").collect();
                let lendat2 = date1ar2.len();
                for indl in 0..lendat2 {
                     let date_int: i32 = date1ar2[indl].clone().parse().unwrap_or(-9999);
                     if date_int == -9999 {
                         bolok = false;
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
                            _ => bolok = false,
                         }
                     }
                }
               if bolok {
                    let datexx = Local.ymd_opt(dateyr, datemo, dateday);
                    if datexx == LocalResult::None {
                        bolok = false;
                    } else {
                        if (datenum < 0) | (datenum > 999) {
                            bolok = false;
                        } else if (datehr < 0) | (datehr > 23) {
                            bolok = false;
                        } else if (datemin < 0) | (datemin > 59) {
                            bolok = false;
                        } else if (datesec < 0) | (datesec > 59) {
                            bolok = false;
                        }
                   }
                }
// date in name end
                if bolok {
                    dateto = Utc.ymd(dateyr, datemo, dateday).and_hms_milli(datehr as u32, datemin as u32, datesec as u32, 0);
                    dateto = dateto + Duration::days(dateyr1*365) +
                                      Duration::days(datemo1*30) +
                                      Duration::days(dateday1) +
                                      Duration::hours(datehr1) +
                                      Duration::minutes(datemin1) +
                                      Duration::seconds(datesec1);

                    file_formdate1 = format!("{}", dateto.format("%Y_%m_%d_%H_%M_%S"));
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* org compare: BAD DATE is not correct for 1 **********</span>");
                }
            } else {
                let mut listdate11: Vec<&str> = filedate1[9..19].split("-").collect();
                let mut listdate12: Vec<&str> = filedate1[20..28].split(":").collect();
                listdate11.append(&mut listdate12);
                let lendate11 = listdate11.len();
                for indlfd11 in 0..lendate11 {
                     let datefd11_int: i32 = listdate11[indlfd11].clone().parse().unwrap_or(-9999);
                     if datefd11_int == -9999 {
                         bolok = false;
                         break;
                     } else {
                         match indlfd11 {
                            0 => dateyr11 = datefd11_int,
                            1 => datemo11 = datefd11_int as u32,
                            2 => dateday11 = datefd11_int as u32,
                            3 => datehr11 = datefd11_int as u32,
                            4 => datemin11 = datefd11_int as u32,
                            5 => datesec11 = datefd11_int as u32,
                            _ => bolok = false,
                         }
                     }
                }
                if bolok {
                    let mut datetox11 = Utc.ymd(2000,1,1).and_hms_milli(1,1,1,0);    
                    datetox11 = Utc.ymd(dateyr11, datemo11, dateday11).and_hms_milli(datehr11, datemin11, datesec11, 0);
                    datetox11 = datetox11 + Duration::days(dateyr1*365) +
                                                       Duration::days(datemo1*30) +
                                                       Duration::days(dateday1) +
                                                       Duration::hours(datehr1) +
                                                       Duration::minutes(datemin1) +
                                                       Duration::seconds(datesec1);
                    file_formdate1 = format!("{}", datetox11.format("%Y_%m_%d_%H_%M_%S"));
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* org compare: BAD DATE is not correct for 1 **********</span>");
                }
            }
        }
        if bolok {
            if odir2chkbox_check.get_active() {
                let yfilename: &String = &format!("{}", tofilename2);
// date in name start
                let date1ar22: Vec<&str> = yfilename[0..23].split("_").collect();
                let lendat22 = date1ar22.len();
                for indl2 in 0..lendat22 {
                     let date_int2: i32 = date1ar22[indl2].clone().parse().unwrap_or(-9999);
                     if date_int2 == -9999 {
                         bolok = false;
                         break;
                     } else {
                         match indl2 {
                            0 => dateyr = date_int2,
                            1 => datemo = date_int2 as u32,
                            2 => dateday = date_int2 as u32,
                            3 => datehr = date_int2,
                            4 => datemin = date_int2,
                            5 => datesec = date_int2,
                            6 => datenum = date_int2,
                            _ => bolok = false,
                         }
                     }
                }
                if bolok {
                    let date22 = Local.ymd_opt(dateyr, datemo, dateday);
                    if date22 == LocalResult::None {
                        bolok = false;
                    } else {
                        if (datenum < 0) | (datenum > 999) {
                            bolok = false;
                        } else if (datehr < 0) | (datehr > 23) {
                            bolok = false;
                        } else if (datemin < 0) | (datemin > 59) {
                            bolok = false;
                        } else if (datesec < 0) | (datesec > 59) {
                            bolok = false;
                        }
                   }
                }
// date in name end
                if bolok {
                    dateto = Utc.ymd(dateyr, datemo, dateday).and_hms_milli(datehr as u32, datemin as u32, datesec as u32, 0);
                    dateto = dateto + Duration::days(dateyr2*365) +
                                      Duration::days(datemo2*30) +
                                      Duration::days(dateday2) +
                                      Duration::hours(datehr2) +
                                      Duration::minutes(datemin2) +
                                      Duration::seconds(datesec2);

                    file_formdate2 = format!("{}", dateto.format("%Y_%m_%d_%H_%M_%S"));
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* org compare: BAD DATE is not correct for 2 **********</span>");
                    bolok = false;
                }
            } else {
                let mut listdate21: Vec<&str> = filedate2[9..19].split("-").collect();
                let mut listdate22: Vec<&str> = filedate2[20..28].split(":").collect();
                listdate21.append(&mut listdate22);
                let lendate21 = listdate21.len();
                for indlfd21 in 0..lendate21 {
                     let datefd21_int: i32 = listdate21[indlfd21].clone().parse().unwrap_or(-9999);
                     if datefd21_int == -9999 {
                         bolok = false;
                         break;
                     } else {
                         match indlfd21 {
                            0 => dateyr21 = datefd21_int,
                            1 => datemo21 = datefd21_int as u32,
                            2 => dateday21 = datefd21_int as u32,
                            3 => datehr21 = datefd21_int as u32,
                            4 => datemin21 = datefd21_int as u32,
                            5 => datesec21 = datefd21_int as u32,
                            _ => bolok = false,
                         }
                     }
                }
                if bolok {
                    let mut datetox21 = Utc.ymd(2000,1,1).and_hms_milli(1,1,1,0);    
                    datetox21 = Utc.ymd(dateyr21, datemo21, dateday21).and_hms_milli(datehr21, datemin21, datesec21, 0);
                    datetox21 = datetox21 + Duration::days(dateyr2*365) +
                                                       Duration::days(datemo2*30) +
                                                       Duration::days(dateday2) +
                                                       Duration::hours(datehr2) +
                                                       Duration::minutes(datemin2) +
                                                       Duration::seconds(datesec2);
                    file_formdate2 = format!("{}", datetox21.format("%Y_%m_%d_%H_%M_%S"));
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* org compare: BAD DATE is not correct for 2 **********</span>");
                }
            }
        }
        if bolok {
            let date1ar111: Vec<&str> = file_formdate1.split("_").collect();
            let lendat111 = date1ar111.len();
            for indl111 in 0..lendat111 {
                 let date_int111: i32 = date1ar111[indl111].clone().parse().unwrap_or(-9999);
                 if date_int111 == -9999 {
                     bolok = false;
                     break;
                 } else {
                     match indl111 {
                       0 => dateyr111 = date_int111,
                       1 => datemo111 = date_int111 as u32,
                       2 => dateday111 = date_int111 as u32,
                       3 => datehr111 = date_int111 as i64,
                       4 => datemin111 = date_int111 as i64,
                       5 => datesec111 = date_int111 as i64,
                       _ => bolok = false,
                     }
                 }
            }
            if bolok {
                let date111 = Local.ymd_opt(dateyr111, datemo111, dateday111);
                if date111 == LocalResult::None {
                    bolok = false;
                } else {
                    if (datehr111 < 0) | (datehr111 > 23) {
                        bolok = false;
                    } else if (datemin111 < 0) | (datemin111 > 59) {
                        bolok = false;
                    } else if (datesec111 < 0) | (datesec111 > 59) {
                        bolok = false;
                    }
                }
            }
            if !bolok {
                messageval_label.set_markup("<span color=\"#FF000000\">********* org compare: BAD DATE is not correct for 1 xx **********</span>");
            }
        }
        if bolok {
            let date1ar222: Vec<&str> = file_formdate2.split("_").collect();
            let lendat222 = date1ar222.len();
            for indl222 in 0..lendat222 {
                 let date_int222: i32 = date1ar222[indl222].clone().parse().unwrap_or(-9999);
                 if date_int222 == -9999 {
                     bolok = false;
                     break;
                 } else {
                     match indl222 {
                       0 => dateyr222 = date_int222,
                       1 => datemo222 = date_int222 as u32,
                       2 => dateday222 = date_int222 as u32,
                       3 => datehr222 = date_int222 as i64,
                       4 => datemin222 = date_int222 as i64,
                       5 => datesec222 = date_int222 as i64,
                       _ => bolok = false,
                     }
                 }
            }
            if bolok {
                let date222 = Local.ymd_opt(dateyr222, datemo222, dateday222);
                if date222 == LocalResult::None {
                    bolok = false;
                } else {
                    if (datehr222 < 0) | (datehr222 > 23) {
                        bolok = false;
                    } else if (datemin222 < 0) | (datemin222 > 59) {
                        bolok = false;
                    } else if (datesec222 < 0) | (datesec222 > 59) {
                        bolok = false;
                    }
                }
            }
            if !bolok {
                messageval_label.set_markup("<span color=\"#FF000000\">********* org compare: BAD DATE is not correct for 2 xx **********</span>");
            }
        }
        if bolok {
            let diffyr1 = dateyr222 as i64 - dateyr111 as i64;
            let diffmo1 = datemo222 as i64 - datemo111 as i64;
            let diffday1 = dateday222 as i64 - dateday111 as i64;
            let diffhr1 = datehr222 - datehr111;
            let diffmin1 = datemin222 - datemin111;
            let diffsec1 = datesec222 - datesec111;
            let diffyr2 = dateyr111 as i64 - dateyr222 as i64;
            let diffmo2 = datemo111 as i64 - datemo222 as i64;
            let diffday2 = dateday111 as i64 - dateday222 as i64;
            let diffhr2 = datehr111 - datehr222;
            let diffmin2 = datemin111 - datemin222;
            let diffsec2 = datesec111 - datesec222;
            let label1 = format!("{}:{}:{}:{}:{}:{}", diffyr1, diffmo1, diffday1, diffhr1, diffmin1, diffsec1);
            let label2 = format!("{}:{}:{}:{}:{}:{}", diffyr2, diffmo2, diffday2, diffhr2, diffmin2, diffsec2);
            odate1_label.set_text(&label1);
            odate2_label.set_text(&label2);
            messageval_label.set_text("organize compare completed")
        }
    }));

//----------------- org date compare button end -----------------------------------

//----------------- Merge From directory button start -----------------------------------
    
//    let window_weakmf = window.downgrade();    
//    let mdirectoryfrom_combobox_weakmf = mdirectoryfrom_combobox.downgrade();
//    let mdirectoryto_combobox_weakmf = mdirectoryto_combobox.downgrade();
//    let messageval_label_weakmf = messageval_label.downgrade();
//    let miconfrom_view_weakmf = miconfrom_view.downgrade();
//    let mimsize_entry_weakmf = mimsize_entry.downgrade();

//    let window_clonemf = window.clone();    
//    let mdirectoryfrom_combobox_clonemf = mdirectoryfrom_combobox.clone();
//    let mdirectoryto_combobox_clonemf = mdirectoryto_combobox.clone();
//    let messageval_label_clonemf = messageval_label.clone();
//    let miconfrom_view_clonemf = miconfrom_view.clone();
//    let mimsize_entry_clonemf = mimsize_entry.clone();
//    mdirectoryfrom_button.connect_clicked( move|_| {

    mdirectoryfrom_button.connect_clicked(glib::clone!(@weak window, @weak mdirectoryfrom_combobox, @weak mdirectoryto_combobox, @weak messageval_label, @weak miconfrom_view, @weak mimsize_entry => move|_| {
    
//    mdirectoryfrom_button.connect_clicked(clone!(window_weakmf, mdirectoryfrom_combobox_weakmf, mdirectoryto_combobox_weakmf, messageval_label_weakmf, miconfrom_view_weakmf, mimsize_entry_weakmf => move|_| {
//        let window = upgrade_weak!(window_weakmf);
//        let mdirectoryfrom_combobox = upgrade_weak!(mdirectoryfrom_combobox_weakmf);
//        let mdirectoryto_combobox = upgrade_weak!(mdirectoryto_combobox_weakmf);
//        let messageval_label = upgrade_weak!(messageval_label_weakmf);
//        let miconfrom_view = upgrade_weak!(miconfrom_view_weakmf);
//        let mimsize_entry = upgrade_weak!(mimsize_entry_weakmf);

        let mut badsize_int = 1;
        let mut icon_int1 = 0;
        let inputic_text = mimsize_entry.get_text();
        let icon_int: i32 = inputic_text.parse().unwrap_or(-99);
        if icon_int > 0 {
            badsize_int = 0;
            icon_int1 = icon_int;
        } else if icon_int == -99 {
            messageval_label.set_markup("<span color=\"#FF000000\">********* List: Icon Size is not an integer **********</span>");
        } else {
            messageval_label.set_markup("<span color=\"#FF000000\">********* List: Icon Size not positive integer **********</span>");
        }
        if badsize_int == 0 {
            if (icon_int1 < 50) | (icon_int1 > 255) {
                messageval_label.set_markup("<span color=\"#FF000000\">********* List: Icon Size not between 50 and 255 **********</span>");
                badsize_int = 1;
            }
        }
        if badsize_int == 0 {
            let dialog = FileChooserDialog::new(
                 Some("Choose From Directory"),
                 Some(&window.clone()),
                 FileChooserAction::SelectFolder,
                 &[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
            );

            dialog.connect_response(move |d: &FileChooserDialog, response: gtk::ResponseType| {
                if response == gtk::ResponseType::Ok {
                    if let Some(foldername) = d.get_file() {
                        if let Some(folderpath) = foldername.get_path() {
                            mdirectoryfrom_combobox.prepend_text(&folderpath.display().to_string());
                            mdirectoryfrom_combobox.set_active(Some(0));
                            let mut listfull: Vec<String> = Vec::new();
                            let mut listname: Vec<String> = Vec::new();
                            let current_dir = folderpath;
                            for entry1 in fs::read_dir(&current_dir).unwrap() {
                                 let entry = entry1.unwrap();
                                 if let Ok(metadata) = entry.metadata() {
                                     if let Ok(file_name) = entry.file_name().into_string() {
                                         if metadata.is_file() {
                                             if file_name.ends_with(".jpg") | file_name.ends_with(".JPG") |
                                                 file_name.ends_with(".jpeg") |file_name.ends_with(".JPEG") |
                                                 file_name.ends_with(".png") |file_name.ends_with(".PNG") { 
                                                 listname.push(file_name.clone());
                                                 let file_path = entry.path().into_os_string().into_string().unwrap();
                                                 listfull.push(file_path.clone());
                                             }
                                         }
                                     }
                                 }
                           }
                           if listname.len() < 1 as usize {
                               let msgstr = format!("<span color=\"#FF000000\">********* From Directory: NO IMAGES TO MERGE **********</span>");
                               messageval_label.set_markup(&msgstr);
                           } else {
                               listfull.sort();
                               listname.sort();
                               let listnamelen = listname.len();
                               let new_model = ListStore::new(&[Pixbuf::static_type(), String::static_type()]);
                               let newtoi = listnamelen as i32 ;
                               for indexi in 0..newtoi {
                                    let file_pathx = &listfull[indexi as usize];
                                    let pixbufx = Pixbuf::from_file(&file_pathx).unwrap();
                                    let mut pixheight = pixbufx.get_height();
                                    let mut pixwidth = pixbufx.get_width();
                                    if pixheight > pixwidth {
                                        pixwidth = icon_int1 * pixwidth / pixheight;
                                        pixheight = icon_int1;
                                    } else {
                                        pixheight = icon_int1 * pixheight / pixwidth;
                                        pixwidth = icon_int1;
                                    }
                                    let pixbuficon = pixbufx.scale_simple(pixwidth, pixheight, gtk::gdk_pixbuf::InterpType::Bilinear);
                                    new_model.insert_with_values(None,
                                          &[FIRST_COL as u32, SECOND_COL as u32],
                                          &[&pixbuficon, &listname[indexi as usize]]);
                               }
                               miconfrom_view.set_model(Some(&new_model));
                               let msgstr = format!("merge total from files {}", listnamelen);
                               messageval_label.set_text(&msgstr);
                           }
                        } else { 
                            messageval_label.set_markup("<span color=\"#FF000000\">********* From Directory: ERROR GETTING PATH **********</span>");
                        }
                    } else { 
                        messageval_label.set_markup("<span color=\"#FF000000\">********* From Directory: ERROR GETTING FILE **********</span>");
                    }
                } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* From Directory: ERROR OK button not selected **********</span>");
                }

                d.close();
            });

//            let dialog = FileChooserDialog::new(Some("Choose From Directory"), Some(&window.clone()), FileChooserAction::SelectFolder);
//            dialog.add_button("Cancel", gtk::ResponseType::Cancel);
//            dialog.add_button("Accept", gtk::ResponseType::Accept);
//            if let Some(cur_dir) = mdirectoryfrom_combobox.get_active_text() {
//                let current_dir = PathBuf::from(&cur_dir);
//                let _resultx = dialog.add_shortcut_folder(current_dir);
//            }
//            if let Some(cur_dir1) = mdirectoryto_combobox.get_active_text() {
//                let current_dir1 = PathBuf::from(&cur_dir1);
//                let _resulty = dialog.add_shortcut_folder(current_dir1);
//            }
//            let result = dialog.run();
//            if result == gtk::ResponseType::Accept {
//                if let Some(uri) = dialog.get_uri() {
//                    let foldername = &uri[7..];
//            dialog.close();
        }
    }));
//----------------- Merge From directory button end -----------------------------------

//----------------- Merge To directory button start -----------------------------------
//    let window_weakmt = window.downgrade();    
//    let mdirectoryto_combobox_weakmt = mdirectoryto_combobox.downgrade();
//    let messageval_label_weakmt = messageval_label.downgrade();
//    let mtreeto_view_weakmt = mtreeto_view.downgrade();

//    let window_clonemt = window.clone();    
//    let mdirectoryto_combobox_clonemt = mdirectoryto_combobox.clone();
//    let messageval_label_clonemt = messageval_label.clone();
//    let mtreeto_view_clonemt = mtreeto_view.clone();
//    mdirectoryto_button.connect_clicked( move|_| {

    mdirectoryto_button.connect_clicked(glib::clone!(@weak window, @weak mdirectoryto_combobox, @weak messageval_label, @weak mtreeto_view => move|_| {

//    mdirectoryto_button.connect_clicked(clone!(window_weakmt, mdirectoryto_combobox_weakmt, messageval_label_weakmt, mtreeto_view_weakmt => move|_| {
//        let window = upgrade_weak!(window_weakmt);
//        let mdirectoryto_combobox = upgrade_weak!(mdirectoryto_combobox_weakmt);
//        let messageval_label = upgrade_weak!(messageval_label_weakmt);
//        let mtreeto_view = upgrade_weak!(mtreeto_view_weakmt);

        let dialog = FileChooserDialog::new(
            Some("Choose  To Directory"),
            Some(&window.clone()),
            FileChooserAction::SelectFolder,
            &[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
        );

        dialog.connect_response(move |d: &FileChooserDialog, response: gtk::ResponseType| {
            if response == gtk::ResponseType::Ok {
                if let Some(foldername) = d.get_file() {
                    if let Some(folderpath) = foldername.get_path() {
                        mdirectoryto_combobox.prepend_text(&folderpath.display().to_string());
                        mdirectoryto_combobox.set_active(Some(0));
                        let current_dir = folderpath;
                        let (errcd, errstr, newmodel) = get_tomodel(current_dir);
                        if errcd == 0 {
                            mtreeto_view.set_model(Some(&newmodel));
                            messageval_label.set_text(&errstr);
                        } else {
                            messageval_label.set_markup(&errstr);
                        }
                  } else { 
                        messageval_label.set_markup("<span color=\"#FF000000\">********* To Directory: ERROR GETTING PATH **********</span>");
                    }
                } else { 
                    messageval_label.set_markup("<span color=\"#FF000000\">********* To Directory: ERROR GETTING FILE **********</span>");
                }
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* To Directory: ERROR OK button not selected **********</span>");
            }

            d.close();
        });
        dialog.show();        
//        let dialog = FileChooserDialog::new(Some("Choose To Directory"), Some(&window.clone()), FileChooserAction::SelectFolder);
//        dialog.add_button("Cancel", gtk::ResponseType::Cancel);
//        dialog.add_button("Accept", gtk::ResponseType::Accept);
//        let result = dialog.run();
//        if result == gtk::ResponseType::Accept {
//            if let Some(uri) = dialog.get_uri() {
//                let foldername = &uri[7..];
//                mdirectoryto_combobox.prepend_text(&foldername);
//                mdirectoryto_combobox.set_active(Some(0));
//                let current_dir = PathBuf::from(&foldername);
//                let (errcd, errstr, newmodel) = get_tomodel(current_dir);
//                if errcd == 0 {
//                    mtreeto_view.set_model(Some(&newmodel));
//                    messageval_label.set_text(&errstr);
//                } else {
//                    messageval_label.set_markup(&errstr);
//                }
//            } else {
//                messageval_label.set_markup("<span color=\"#FF000000\">********* To Directory: ERROR GETTING URI **********</span>");
//            }
//        } else {
//            messageval_label.set_markup("<span color=\"#FF000000\">********* To Directory: ERROR accept button not selected **********</span>");
//        }
//        dialog.close();
    }));
//----------------- Merge To directory button end -----------------------------------

//----------------- Merge merge button start -----------------------------------
    
//    let mdirectoryto_combobox_weakmm = mdirectoryto_combobox.downgrade();
//    let miconfrom_view_weakmm = miconfrom_view.downgrade();
//    let mdirectoryfrom_combobox_weakmm = mdirectoryfrom_combobox.downgrade();
//    let mtreeto_view_weakmm = mtreeto_view.downgrade();
//    let mbeforebox_check_weakmm = mbeforebox_check.downgrade();
//    let messageval_label_weakmm = messageval_label.downgrade();

    let mdirectoryto_combobox_clonemm = mdirectoryto_combobox.clone();
    let miconfrom_view_clonemm = miconfrom_view.clone();
    let mdirectoryfrom_combobox_clonemm = mdirectoryfrom_combobox.clone();
    let mtreeto_view_clonemm = mtreeto_view.clone();
    let mbeforebox_check_clonemm = mbeforebox_check.clone();
    let messageval_label_clonemm = messageval_label.clone();
    mmerge_button.connect_clicked(move|_| {
    
//    mmerge_button.connect_clicked(clone!(mdirectoryto_combobox_weakmm, miconfrom_view_weakmm, mdirectoryfrom_combobox_weakmm, mtreeto_view_weakmm, mbeforebox_check_weakmm, messageval_label_weakmm  => move|_| {
//        let mdirectoryto_combobox = upgrade_weak!(mdirectoryto_combobox_weakmm);
//        let miconfrom_view = upgrade_weak!(miconfrom_view_weakmm);
//        let mdirectoryfrom_combobox = upgrade_weak!(mdirectoryfrom_combobox_weakmm);
//        let mtreeto_view = upgrade_weak!(mtreeto_view_weakmm);
//        let mbeforebox_check = upgrade_weak!(mbeforebox_check_weakmm);
//        let messageval_label = upgrade_weak!(messageval_label_weakmm);

        let mut bolok = true;
        let mut bolusenum = false;
        let mut str_cur_dirfrom = format!(" ");
        let mut fromfilename = format!(" ");
        let mut str_cur_dirto = format!(" ");
        let mut tofilename = format!(" ");
        let mut filenamestr = format!(" ");
        let mut filenameother = format!(" ");
        let mut prefixstr = format!(" ");
        let mut dateto = Utc.ymd(2000,1,1).and_hms_milli(1,1,1,0);
        let mut dateother = Utc.ymd(2000,1,1).and_hms_milli(1,1,1,0);
        let mut datenumto = 0;
        let mut datenumother = 9999;
        let mut dateyr = 0;
        let mut datemo = 0;
        let mut dateday = 0;
        let mut datehr = 0;
        let mut datemin = 0;
        let mut datesec = 0;


// check if from directory has images
        let iconmodel1 = miconfrom_view_clonemm.get_model();
        if iconmodel1 == None {
             messageval_label_clonemm.set_markup("<span color=\"#FF000000\">********* Merge: ERROR NOTHING IN FROM DIRECTORY LIST **********</span>");
             bolok = false;
        }

// check if to directory has images
        if bolok {
            let treemodel1 = mtreeto_view_clonemm.get_model();
            if treemodel1 == None {
                messageval_label_clonemm.set_markup("<span color=\"#FF000000\">********* Merge: ERROR NOTHING IN TO DIRECTORY LIST **********</span>");
                bolok = false;
            }
        }

// check if both directories exist and they are not equal
        if bolok {
            if let Some(cur_dir1) = mdirectoryfrom_combobox_clonemm.get_active_text() {
                str_cur_dirfrom = cur_dir1.to_string();
                if let Some(cur_dir2) = mdirectoryto_combobox_clonemm.get_active_text() {
                    str_cur_dirto = cur_dir2.to_string();
                    if str_cur_dirto == str_cur_dirfrom {
                        messageval_label_clonemm.set_markup("<span color=\"#FF000000\">********* Merge: FROM DIR AND TO DIR ARE THE SAME DIRECTORY **********</span>");
                        bolok = false;
                    }
                } else {
                    messageval_label_clonemm.set_markup("<span color=\"#FF000000\">********* Merge: ERROR GETTING TO DIRECTORY IN COMBOBOX **********</span>");
                    bolok = false;
                }
            } else {
                messageval_label_clonemm.set_markup("<span color=\"#FF000000\">********* Merge: ERROR GETTING FROM DIRECTORY IN COMBOBOX **********</span>");
                bolok = false;
            }
        }

// check a from file has been selected and that there is only one selection
        if bolok {
            let icon_selectpath = miconfrom_view_clonemm.get_selected_items();
            if icon_selectpath.len() == 1 {
                let iconmodeluw = iconmodel1.unwrap();
                let iconiter = iconmodeluw.get_iter(&icon_selectpath[0]).unwrap();
                let filenameval = iconmodeluw.get_value(&iconiter,1).get::<String>();
                filenamestr = format!("{:?}", filenameval).to_owned();
                let fileln = filenamestr.len();
                let fileend = fileln - 3;
                let filestart = 9;

// save the from file name
                fromfilename = format!("{:?}", filenamestr.get(filestart..fileend)).to_owned();
                let msgstr = format!("{:?} is the row name selected in From directory", fromfilename);
                messageval_label_clonemm.set_text(&msgstr);
            } else {
                messageval_label_clonemm.set_markup("<span color=\"#FF000000\">********* Merge: NO SELECTION OR TOO MANY SELECTIONS IN  FROM  DIRECTORY **********</span>");
                bolok = false;
            }
        }

// check if to file was selected - no check for number just get first one
        if bolok {
            let selectiont = mtreeto_view_clonemm.get_selection();
            if let Some((modelt, itert)) = selectiont.get_selected() {
                let tofilenameval = modelt.get_value(&itert, 0).get::<String>();
                let filenamestr = format!("{:?}", tofilenameval);
                let tofileln = filenamestr.len();
                let tofileend = tofileln - 3;
                let tofilestart = 9;
// save to file name
                tofilename = format!("{:?}", filenamestr.get(tofilestart..tofileend));
                let msgstr = format!("{:?} is the row selected in To directory", tofilename);
                messageval_label_clonemm.set_text(&msgstr);
            } else {
                messageval_label_clonemm.set_markup("<span color=\"#FF000000\">********* Merge: NO SELECTION IN TO DIRECTORY **********</span>");
                bolok = false;
            }            
        }

// get list of file in to directory and get before or after file name
        if bolok {
            let mut numentry = 0;
            let mut listitems: Vec<String> = Vec::new();
            for entry1 in fs::read_dir(&str_cur_dirto).unwrap() {
                 let entry = entry1.unwrap();
                 if let Ok(metadata) = entry.metadata() {
                     if let Ok(file_name) = entry.file_name().into_string() {
                         if metadata.is_file() {
                             if file_name.ends_with(".jpg") | file_name.ends_with(".JPG") |
                                file_name.ends_with(".jpeg") |file_name.ends_with(".JPEG") |
                                file_name.ends_with(".png") |file_name.ends_with(".PNG") {
                                 listitems.push(file_name);
                                 numentry = numentry + 1;
                             }
                         }
                     }
                 }
            }
            if numentry < 1 {
                messageval_label_clonemm.set_markup("<span color=\"#FF000000\">********* Merge: ERROR getting list of files in to directory **********</span>");
                bolok = false;
            } else {
                listitems.sort();
                let listitemlen = listitems.len();
                let newtoi = listitemlen as i32 ;
                let fileln = tofilename.len();
                let fileend = fileln - 2;
                let filestart = 6;
                let tofilenamex = tofilename.get(filestart..fileend).unwrap();
                let mut namep = " ";
                let mut namec = " ";
                let mut namea = " ";
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
                             if tofilenamex == namep {
                                 namea = namec;
                                 namec = namep;
                                 namep = " ";
                                 found = 1;
                             } else if tofilenamex == namec {
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
                    if tofilenamex == namep {
                        namea = namec;
                        namec = namep;
                        namep = " ";
                        found = 1;
                    } else { 
                        if namea == " " {
                            if tofilenamex == namec {
                                found = 1;
                            }
                        } else {
                            if tofilenamex == namea {
                                namep = namec;
                                namec = namea;
                                namea = " ";
                                found = 1;
                            }
                        }
                    }
                }
                if found == 0 {
                    let msgstr = format!("pick {:?},  p {:?}.  c {:?}, a {:?}  not found", tofilenamex, namep, namec, namea);
                    messageval_label_clonemm.set_text(&msgstr);
//                    messageval_label.set_markup("<span color=\"#FF000000\">********* Preview: selected file not found in list **********</span>");
                    bolok = false;
                } else {
                    if mbeforebox_check_clonemm.get_active() {
                        if namep != " " {
                            datenumother = 0;
                            filenameother = format!("{:?}", namep);
                        }
                    } else {
                        if namea != " " {
                            datenumother = 0;
                            filenameother = format!("{:?}", namea);
                        }
                    }
                    let msgstr = format!("pick {:?},  other {:?} found", tofilenamex, filenameother);
                    messageval_label_clonemm.set_text(&msgstr);
                }
            }
        }

// get dates and validate for to filename and previous or after filename
        if bolok {
            let fileln = tofilename.len();
            let fileend = fileln - 2;
            let filestart = 6;
            let tofilenamex = tofilename.get(filestart..fileend).unwrap();
// date in name start
            let date1ar2: Vec<&str> = tofilenamex[0..23].split("_").collect();
            let lendat2 = date1ar2.len();
            let mut baddate1 = 0;
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
                        6 => datenumto = date_int as i32,
                        _ => baddate1 = 1,
                     }
                 }
            }
            if baddate1 == 0 {
                let datexx = Local.ymd_opt(dateyr, datemo, dateday);
                if datexx == LocalResult::None {
                    baddate1 = 1;
                } else {
                    if (datenumto < 0) | (datenumto > 999) {
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
            if baddate1 == 0 {
                dateto = Utc.ymd(dateyr, datemo, dateday).and_hms_milli(datehr as u32, datemin as u32, datesec as u32, 0);
                let mut dateyro = 0;
                let mut datemoo = 0;
                let mut datedayo = 0;
                let mut datehro = 0;
                let mut datemino = 0;
                let mut dateseco = 0;
                if datenumother < 1000 {
                    let filelno = filenameother.len();
                    let fileendo = filelno - 1;
                    let filestarto = 1;
                    let filenamexo = filenameother.get(filestarto..fileendo).unwrap();
                    let date1ar2o: Vec<&str> = filenamexo[0..23].split("_").collect();
                    let lendat2o = date1ar2o.len();
                    for indlo in 0..lendat2o {
                         let date_into: i32 = date1ar2o[indlo].clone().parse().unwrap_or(-9999);
                         if date_into == -9999 {
                             baddate1 = 1;
                         } else {
                             match indlo {
                               0 => dateyro = date_into,
                               1 => datemoo = date_into as u32,
                               2 => datedayo = date_into as u32,
                               3 => datehro = date_into as i32,
                               4 => datemino = date_into as i32,
                               5 => dateseco = date_into as i32,
                               6 => datenumother = date_into as i32,
                               _ => baddate1 = 1,
                             }
                         }
                    }
                    if baddate1 == 0 {
                        let dateyy = Local.ymd_opt(dateyro, datemoo, datedayo);
                        if dateyy == LocalResult::None {
                            baddate1 = 1;
                        } else {
                            if (datenumother < 0) | (datenumother > 999) {
                                baddate1 = 1;
                            } else if (datehro < 0) | (datehro > 23) {
                                baddate1 = 1;
                            } else if (datemino < 0) | (datemino > 59) {
                                baddate1 = 1;
                            } else if (dateseco < 0) | (dateseco > 59) {
                                baddate1 = 1;
                            }
                        }
                    }
                    if baddate1 == 0 {
                        dateother = Utc.ymd(dateyro, datemoo, datedayo).and_hms_milli(datehro as u32, datemino as u32, dateseco as u32, 0);
                    }
                }
            }
            if baddate1 == 1 {
                let msgstr = format!("pick {:?},  num {:?}.  other {:?},  num {:?}  bad date", tofilename, datenumto, filenameother, datenumother);
                messageval_label_clonemm.set_text(&msgstr);
                bolok = false;
            }
        }

        if bolok {
            if datenumother > 999 {
                if mbeforebox_check_clonemm.get_active() {
                    if datenumto < 2 {
                        messageval_label_clonemm.set_markup("<span color=\"#FF000000\">********* Merge: selected file number is less than 2, first file and before is checked **********</span>");
                        bolok = false;
                    } else {
                        datenumto = datenumto - (datenumto/2);
                        bolusenum = true;
                    }
                } else {
                    if datenumto > 998 {
                        messageval_label_clonemm.set_markup("<span color=\"#FF000000\">********* Merge: selected file number is less than 2, first file and before is checked **********</span>");
                        bolok = false;
                    } else {
                        datenumto = datenumto + ((1000 - datenumto)/2);
                        bolusenum = true;
                    }
                }
            } else {
                if dateto == dateother {
                    if mbeforebox_check_clonemm.get_active() {
                        if (datenumto - datenumother) < 2 {
                            messageval_label_clonemm.set_markup("<span color=\"#FF000000\">********* Merge: before checked & selected file number & previous file number less than 2 apart **********</span>");
                            bolok = false;
                        } else {
                            datenumto = datenumto - ((datenumto - datenumother)/2);
                            bolusenum = true;
                        }
                    } else {
                        if (datenumother - datenumto) < 2 {
                            messageval_label_clonemm.set_markup("<span color=\"#FF000000\">********* Merge: selected file number & next file number less than 2 apart **********</span>");
                            bolok = false;
                        } else {
                            datenumto = datenumto + ((datenumother - datenumto)/2);
                            bolusenum = true;
                        }
                    }
                } else {
                    if mbeforebox_check_clonemm.get_active() {
                        let duration = dateto.signed_duration_since(dateother);
                        if duration.num_seconds() < 2 {
                            if datenumto > 1 {
                                datenumto = datenumto - (datenumto/2);
                                bolusenum = true;
                            } else if datenumto == 1 {
                                datenumto = 0;
                                bolusenum = true;
                            }
                        }
                    } else {                                
                        let durationx = dateother.signed_duration_since(dateto);
                        if durationx.num_seconds() < 2 {
                            if datenumto < 999 {
                                datenumto = datenumto + ((1000 - datenumto)/2);
                                bolusenum = true;
                            }
                        }                               
                    }
                    if !bolusenum {
                        let mut durationn = dateother.signed_duration_since(dateto);
                        if mbeforebox_check.get_active() {
                            durationn = dateto.signed_duration_since(dateother);
                        }
                        if durationn.num_seconds() < 2 {
                            if mbeforebox_check.get_active() {
                                if datenumother > 998 {
                                    messageval_label_clonemm.set_markup("<span color=\"#FF000000\">********* Merge: before checked & selected file number & previous file number too close **********</span>");
                                    bolok = false;
                                } else {
                                    dateto = dateother;
                                    datenumto = datenumother + ((1000 - datenumother)/2);
                                }
                            } else {
                                if datenumother < 1 {
                                    messageval_label_clonemm.set_markup("<span color=\"#FF000000\">********* Merge: before checked & selected file number & previous file number too close **********</span>");
                                    bolok = false;
                                } else {
                                    dateto = dateother;
                                    if datenumother > 1 {
                                        datenumto = datenumother - (datenumother/2);
                                    } else {
                                        datenumto = 0;
                                    }
                                }
                            }
                        } else {
                            datenumto = 500;
                            let sec = durationn.num_seconds();
                            if mbeforebox_check_clonemm.get_active() {
                                dateto = dateother + Duration::seconds(sec/2);
                            } else {
                                dateto = dateother - Duration::seconds(sec/2);
                            }
                        }
                    }
                }
            }
        }
        if bolok {
            if !bolusenum {
                let mut baddate2 = 0;
                let datestr = format!("{}",dateto.format("%Y:%m:%d:%T"));
                let date1ar2d: Vec<&str> = datestr.split(":").collect();
                let lendat2d = date1ar2d.len();
                let mut dateyrd = 0;
                let mut datemod = 0;
                let mut datedayd = 0;
                let mut datehrd = 0;
                let mut datemind = 0;
                let mut datesecd = 0;
                for indld in 0..lendat2d {
                     let date_intd: i32 = date1ar2d[indld].clone().parse().unwrap_or(-9999);
                     if date_intd == -9999 {
                         baddate2 = 1;
                     } else {
                         match indld {
                            0 => dateyrd = date_intd,
                            1 => datemod = date_intd as u32,
                            2 => datedayd = date_intd as u32,
                            3 => datehrd = date_intd as u32,
                            4 => datemind = date_intd as u32,
                            5 => datesecd = date_intd as u32,
                            _ => baddate2 = 1,
                         }
                    }
                }
                if baddate2 == 1 {
                    let msgstr = format!("pick {:?},  num {:?}.  date {:?} bad date format", tofilename, datenumto, dateto.format("%Y:%m:%d:%T"));
                    messageval_label_clonemm.set_text(&msgstr);
                    bolok = false;
                } else {
                    prefixstr = format!("{}_{:02}_{:02}_{:02}_{:02}_{:02}_{:03}_", dateyrd, datemod, datedayd, datehrd, datemind, datesecd, datenumto);
                    let msgstr = format!("pick {:?},  num {:?}.  date {:?} good format", tofilename, datenumto, prefixstr);
                    messageval_label_clonemm.set_text(&msgstr);
                }
            } else {
                let datesubstr = &tofilename[6..25];
                prefixstr = format!("{}_{:03}_", datesubstr, datenumto);
                let msgstr = format!("pick {:?},  num {:?}.  date {:?} good format", tofilename, datenumto, prefixstr);
                messageval_label_clonemm.set_text(&msgstr);
            }
        }
        if bolok {
            let fileln = tofilename.len();
            let fileend = fileln - 2;
            let datesubstr = &tofilename[30..fileend];
            let strlento = datesubstr.len();
            let filelnf = fromfilename.len();
            let fileendf = filelnf - 2;
            let mut datesubstrf: String = fromfilename[6..fileendf].to_owned();
            let strlenfrom = datesubstrf.len();
            if strlenfrom < strlento {
                let mut prefixx: String = "x".to_owned();
                for _numx in 0..(strlento - strlenfrom - 1) {
                     prefixx.push_str("x");
                }
                datesubstrf = format!("{}{}", &prefixx, &datesubstrf);
            } else {
                datesubstrf = datesubstrf[(strlenfrom - strlento)..].to_string();
            }
            let re = Regex::new(r"[^A-Za-z0-9.]").unwrap();
            let after = re.replace_all(&datesubstrf, "_");
            let datesubstrfx = after.to_string();
            let filelnxx = fromfilename.len();
            let fileendxx = filelnxx - 2;
            let filestartxx = 6;
            let fromfilenamexx = fromfilename.get(filestartxx..fileendxx).unwrap();
            let msgstr = format!("copied {} to {}{}", fromfilenamexx, prefixstr, datesubstrfx);            
            let fullfrom = str_cur_dirfrom + "/" + &fromfilenamexx;
            let fullto = str_cur_dirto.clone() + "/" + &prefixstr + &datesubstrfx;
            let _output = Command::new("cp")
                                  .arg("-p")
                                  .arg(&fullfrom)
                                  .arg(&fullto)
                                  .output()
                                  .expect("failed to execute process");

            let current_dir = PathBuf::from(&str_cur_dirto);
            let (errcd, errstr, newmodel) = get_tomodel(current_dir);
            if errcd == 0 {
                mtreeto_view_clonemm.set_model(Some(&newmodel));
                messageval_label_clonemm.set_text(&msgstr);
            } else {
                messageval_label_clonemm.set_markup(&errstr);
            }
        }
    });

//----------------- Merge merge button end -----------------------------------

//----------------- Merge preview button start -----------------------------------
//    let mdirectoryto_combobox_weakmp = mdirectoryto_combobox.downgrade();
//    let mtreeto_view_weakmp = mtreeto_view.downgrade();
//    let mprevimageto_label_weakmp = mprevimageto_label.downgrade();
//    let mcurrimageto_label_weakmp = mcurrimageto_label.downgrade();
//    let mafterimageto_label_weakmp = mafterimageto_label.downgrade();
//    let messageval_label_weakmp = messageval_label.downgrade();
//    let mimsize_entry_weakmp = mimsize_entry.downgrade();
    
    let mdirectoryto_combobox_clonemp = mdirectoryto_combobox.clone();
    let mtreeto_view_clonemp = mtreeto_view.clone();
    let mprevimageto_label_clonemp = mprevimageto_label.clone();
    let mcurrimageto_label_clonemp = mcurrimageto_label.clone();
    let mafterimageto_label_clonemp = mafterimageto_label.clone();
    let messageval_label_clonemp = messageval_label.clone();
    let mimsize_entry_clonemp = mimsize_entry.clone();
    mpreview_button.connect_clicked(move|_| {
//    mpreview_button.connect_clicked(clone!(mdirectoryto_combobox_weakmp, mtreeto_view_weakmp, mprevimageto_label_weakmp, mcurrimageto_label_weakmp, mafterimageto_label_weakmp, messageval_label_weakmp, mimsize_entry_weakmp  => move|_| {
//        let mdirectoryto_combobox = upgrade_weak!(mdirectoryto_combobox_weakmp);
//        let mtreeto_view = upgrade_weak!(mtreeto_view_weakmp);
//        let mprevimageto_label = upgrade_weak!(mprevimageto_label_weakmp);
//        let mcurrimageto_label = upgrade_weak!(mcurrimageto_label_weakmp);
//        let mafterimageto_label = upgrade_weak!(mafterimageto_label_weakmp);
//        let messageval_label = upgrade_weak!(messageval_label_weakmp);
//        let mimsize_entry = upgrade_weak!(mimsize_entry_weakmp);

        let mut badsize_int = 1;
        let mut icon_int1 = 0;

        let inputic_text = mimsize_entry_clonemp.get_text();
        let icon_int: i32 = inputic_text.parse().unwrap_or(-99);
        if icon_int > 0 {
            badsize_int = 0;
            icon_int1 = icon_int;
        } else if icon_int == -99 {
            messageval_label_clonemp.set_markup("<span color=\"#FF000000\">********* Preview: Icon Size is not an integer **********</span>");
        } else {
            messageval_label_clonemp.set_markup("<span color=\"#FF000000\">********* Preview: Icon Size not positive integer **********</span>");
        }
        if badsize_int == 0 {
            if (icon_int1 < 50) | (icon_int1 > 255) {
                messageval_label_clonemp.set_markup("<span color=\"#FF000000\">********* Preview: Icon Size not between 50 and 255 **********</span>");
                badsize_int = 1;
            }
        }
        if badsize_int == 0 {
            if let Some(cur_dir2) = mdirectoryto_combobox_clonemp.get_active_text() {
                let str_cur_dir2 = &cur_dir2;
                let treemodel1 = mtreeto_view_clonemp.get_model();
                if treemodel1 == None {
                    messageval_label_clonemp.set_markup("<span color=\"#FF000000\">********* Preview: ERROR NOTHING IN TO DIRECTORY LIST **********</span>");
                } else {
                    let selectiont = mtreeto_view_clonemp.get_selection();
                    if let Some((modelt, itert)) = selectiont.get_selected() {
                        let tofilenameval = modelt.get_value(&itert, 0).get::<String>();
                        let tofilenamestr = format!("{:?}", tofilenameval);
                        let tofileln = tofilenamestr.len();
                        let tofileend = tofileln - 3;
                        let tofilestart = 9;
                        let tofilename: String = tofilenamestr.get(tofilestart..tofileend).unwrap().to_string();
                        let tofilenamex: String = tofilenamestr.get(tofilestart..tofileend).unwrap().to_string();
                        let tofilenamey: String = tofilenamestr.get(tofilestart..tofileend).unwrap().to_string();
                        let (errcd, errstr, namep, namea) = get_prevafter(str_cur_dir2.to_string(), tofilename);
                        if errcd > 0 {
                            messageval_label_clonemp.set_markup(&errstr);
                        } else {
                            let fullnamec = format!("{}/{}", &str_cur_dir2, &tofilenamex.to_string());
                            let pixbufxc = Pixbuf::from_file(&fullnamec).unwrap();
                            let mut pixheight = pixbufxc.get_height();
                            let mut pixwidth = pixbufxc.get_width();
                            if pixheight > pixwidth {
                                pixwidth = icon_int1 * pixwidth / pixheight;
                                pixheight = icon_int1;
                            } else {
                                pixheight = icon_int1 * pixheight / pixwidth;
                                pixwidth = icon_int1;
                            }
                            let pixbuficonc: Pixbuf = pixbufxc.scale_simple(pixwidth, pixheight, gtk::gdk_pixbuf::InterpType::Bilinear).unwrap();
                            mcurrimageto_label_clonemp.set_from_pixbuf(Some(&pixbuficonc));
                            if namep == " " {
                                mprevimageto_label_clonemp.clear();
                            } else {
                                let fullnamep = format!("{}/{}", &str_cur_dir2, &namep);
                                let pixbufxp = Pixbuf::from_file(&fullnamep).unwrap();
                                pixheight = pixbufxp.get_height();
                                pixwidth = pixbufxp.get_width();
                                if pixheight > pixwidth {
                                    pixwidth = icon_int1 * pixwidth / pixheight;
                                    pixheight = icon_int1;
                                } else {
                                    pixheight = icon_int1 * pixheight / pixwidth;
                                    pixwidth = icon_int1;
                                }
                                let pixbuficonp: Pixbuf = pixbufxp.scale_simple(pixwidth, pixheight, gtk::gdk_pixbuf::InterpType::Bilinear).unwrap();
                                mprevimageto_label_clonemp.set_from_pixbuf(Some(&pixbuficonp));
                            }
                            if namea == " " {
                                mafterimageto_label_clonemp.clear();
                            } else {
                                let fullnamea = format!("{}/{}", &str_cur_dir2, &namea);
                                let pixbufxa = Pixbuf::from_file(&fullnamea).unwrap();
                                pixheight = pixbufxa.get_height();
                                pixwidth = pixbufxa.get_width();
                                if pixheight > pixwidth {
                                    pixwidth = icon_int1 * pixwidth / pixheight;
                                    pixheight = icon_int1;
                                } else {
                                    pixheight = icon_int1 * pixheight / pixwidth;
                                    pixwidth = icon_int1;
                                }
                                let pixbuficona: Pixbuf = pixbufxa.scale_simple(pixwidth, pixheight, gtk::gdk_pixbuf::InterpType::Bilinear).unwrap();
                                mafterimageto_label_clonemp.set_from_pixbuf(Some(&pixbuficona));
                            }
                            let msgstr = format!("merge Previewed: {} -- {} -- {}", namep, tofilenamey, namea);
                            messageval_label_clonemp.set_text(&msgstr);
                        }
                    } else {
                        messageval_label_clonemp.set_markup("<span color=\"#FF000000\">********* Preview: NO SELECTION IN TO DIRECTORY **********</span>");
                    }            
                }
            } else {
                messageval_label_clonemp.set_markup("<span color=\"#FF000000\">********* Preview: ERROR GETTING TO DIRECTORY IN COMBOBOX **********</span>");
            }
        }
    });

//----------------- Merge preview button end -----------------------------------

//----------------- convert directory  button start -----------------------------------
//    let window_weakcd = window.downgrade();    
//    let cdirectory1_combobox_weakcd = cdirectory1_combobox.downgrade();
//    let messageval_label_weakcd = messageval_label.downgrade();
//    let ctree_view1_weakcd = ctree_view1.downgrade();

//    let window_clonecd = window.clone();    
//    let cdirectory1_combobox_clonecd = cdirectory1_combobox.clone();
//    let messageval_label_clonecd = messageval_label.clone();
//    let ctree_view1_clonecd = ctree_view1.clone();
//    cdirectory1_button.connect_clicked( move|_| {

    cdirectory1_button.connect_clicked(glib::clone!(@weak window, @weak cdirectory1_combobox, @weak messageval_label, @weak ctree_view1 => move|_| {

//    cdirectory1_button.connect_clicked(clone!(window_weakcd, cdirectory1_combobox_weakcd, messageval_label_weakcd, ctree_view1_weakcd => move|_| {
//        let window = upgrade_weak!(window_weakcd);
//        let cdirectory1_combobox = upgrade_weak!(cdirectory1_combobox_weakcd);
//        let messageval_label = upgrade_weak!(messageval_label_weakcd);
//        let ctree_view1 = upgrade_weak!(ctree_view1_weakcd);
        
        let dialog = FileChooserDialog::new(
            Some("Choose a Directory"),
            Some(&window.clone()),
            FileChooserAction::SelectFolder,
            &[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
        );

        dialog.connect_response(move |d: &FileChooserDialog, response: gtk::ResponseType| {
            if response == gtk::ResponseType::Ok {
                if let Some(foldername) = d.get_file() {
                    if let Some(folderpath) = foldername.get_path() {
                        cdirectory1_combobox.prepend_text(&folderpath.display().to_string());
                        cdirectory1_combobox.set_active(Some(0));
                        let current_dir = folderpath;
                        let (errcd, errstr, newmodel) = get_dirmodel(current_dir);
                        if errcd == 0 {
                            ctree_view1.set_model(Some(&newmodel));
                            messageval_label.set_text(&errstr);
                        } else {
                            messageval_label.set_markup(&errstr);
                        }
                    } else { 
                        messageval_label.set_markup("<span color=\"#FF000000\">********* Directory: ERROR GETTING PATH **********</span>");
                    }
                } else { 
                    messageval_label.set_markup("<span color=\"#FF000000\">********* Directory: ERROR GETTING FILE **********</span>");
                }
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* Directory: ERROR OK button not selected **********</span>");
            }

            d.close();
        });
        dialog.show();        

//        let dialog = FileChooserDialog::new(Some("Choose a Directory"), Some(&window.clone()), FileChooserAction::SelectFolder);
//        dialog.add_button("Cancel", gtk::ResponseType::Cancel);
//        dialog.add_button("Accept", gtk::ResponseType::Accept);
//        let result = dialog.run();
//        if result == gtk::ResponseType::Accept {
//           if let Some(uri) = dialog.get_uri() {
//                let foldername = &uri[7..];
//                cdirectory1_combobox.prepend_text(&foldername);
//                cdirectory1_combobox.set_active(Some(0));
//                let current_dir = PathBuf::from(&foldername);
//                let (errcd, errstr, newmodel) = get_dirmodel(current_dir);
//                if errcd == 0 {
//                    ctree_view1.set_model(Some(&newmodel));
//                    messageval_label.set_text(&errstr);
//                } else {
//                    messageval_label.set_markup(&errstr);
//                }
//            } else { 
//                messageval_label.set_markup("<span color=\"#FF000000\">********* Directory : ERROR GETTING URI **********</span>");
//            }
//        } else {
//            messageval_label.set_markup("<span color=\"#FF000000\">********* Directory : ERROR accept button not selected **********</span>");
//        }
//        dialog.close();

    }));
//----------------- convert directory  button end -----------------------------------

//----------------- convert out directory button start -----------------------------------
    
//    let window_weakcd = window.downgrade();    
//    let cdirectory_o_combobox_weakcd = cdirectory_o_combobox.downgrade();
//    let messageval_label_weakcd = messageval_label.downgrade();

//    let window_clonecd = window.clone();    
//    let cdirectory_o_combobox_clonecd = cdirectory_o_combobox.clone();
//    let messageval_label_clonecd = messageval_label.clone();
//    cdirectory_o_button.connect_clicked( move|_| {

    cdirectory_o_button.connect_clicked(glib::clone!(@weak window, @weak cdirectory_o_combobox, @weak messageval_label => move|_| {
    
//    cdirectory_o_button.connect_clicked(clone!(window_weakcd, cdirectory_o_combobox_weakcd, messageval_label_weakcd => move|_| {
//        let window = upgrade_weak!(window_weakcd);
//        let cdirectory_o_combobox = upgrade_weak!(cdirectory_o_combobox_weakcd);
//        let messageval_label = upgrade_weak!(messageval_label_weakcd);
        
        let dialog = FileChooserDialog::new(
            Some("Choose a Directory"),
            Some(&window.clone()),
            FileChooserAction::SelectFolder,
            &[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
        );

        dialog.connect_response(move |d: &FileChooserDialog, response: gtk::ResponseType| {
            if response == gtk::ResponseType::Ok {
                if let Some(foldername) = d.get_file() {
                    if let Some(folderpath) = foldername.get_path() {
                        cdirectory_o_combobox.prepend_text(&folderpath.display().to_string());
                        cdirectory_o_combobox.set_active(Some(0));
                        messageval_label.set_text("convert out directory selected");
                    } else { 
                        messageval_label.set_markup("<span color=\"#FF000000\">********* convert Out Directory: ERROR GETTING PATH **********</span>");
                    }
                } else { 
                    messageval_label.set_markup("<span color=\"#FF000000\">********* convert Out Directory: ERROR GETTING FILE **********</span>");
                }
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* convert Out Directory: ERROR OK button not selected **********</span>");
            }

            d.close();
        });
        dialog.show();        

//        let dialog = FileChooserDialog::new(Some("Choose a Directory"), Some(&window.clone()), FileChooserAction::SelectFolder);
//        dialog.add_button("Cancel", gtk::ResponseType::Cancel);
//        dialog.add_button("Accept", gtk::ResponseType::Accept);
//        let result = dialog.run();
//        if result == gtk::ResponseType::Accept {
//            if let Some(uri) = dialog.get_uri() {
//                let foldername = &uri[7..];
//                cdirectory_o_combobox.prepend_text(&foldername);
//                cdirectory_o_combobox.set_active(Some(0));
//                messageval_label.set_text("convert out directory selected");
//            } else { 
//                messageval_label.set_markup("<span color=\"#FF000000\">********* convert Out Directory: ERROR GETTING URI **********</span>");
//            }
//        } else {
//        messageval_label.set_markup("<span color=\"#FF000000\">********* convert Out Directory: ERROR accept button not selected **********</span>");
//        }
//        dialog.close();
    }));
//----------------- convert out directory button end -----------------------------------

//----------------- convert merge button start -----------------------------------
    
//    let cdirectory1_combobox_weakcm = cdirectory1_combobox.downgrade();
//    let ctree_view1_weakcm = ctree_view1.downgrade();
//    let ctree_view3_weakcm = ctree_view3.downgrade();
//    let cfilesize_entry_weakcm = cfilesize_entry.downgrade();
//    let cdatemod1_entry_weakcm = cdatemod1_entry.downgrade();
//    let messageval_label_weakcm = messageval_label.downgrade();
    
    let cdirectory1_combobox_clonecm = cdirectory1_combobox.clone();
    let ctree_view1_clonecm = ctree_view1.clone();
    let ctree_view3_clonecm = ctree_view3.clone();
    let cfilesize_entry_clonecm = cfilesize_entry.clone();
    let cdatemod1_entry_clonecm = cdatemod1_entry.clone();
    let messageval_label_clonecm = messageval_label.clone();
    cmerge_button.connect_clicked( move|_| {

//    cmerge_button.connect_clicked(clone!(cdirectory1_combobox_weakcm, ctree_view1_weakcm, ctree_view3_weakcm, cfilesize_entry_weakcm, cdatemod1_entry_weakcm, messageval_label_weakcm  => move|_| {
//        let cdirectory1_combobox = upgrade_weak!(cdirectory1_combobox_weakcm);
//        let ctree_view1 = upgrade_weak!(ctree_view1_weakcm);
//        let ctree_view3 = upgrade_weak!(ctree_view3_weakcm);
//        let cfilesize_entry = upgrade_weak!(cfilesize_entry_weakcm);
//       let cdatemod1_entry = upgrade_weak!(cdatemod1_entry_weakcm);
//        let messageval_label = upgrade_weak!(messageval_label_weakcm);

        let mut str_cur_dir1 = format!(" ");
        let mut dateyr1 = 0;
        let mut datemo1 = 0;
        let mut dateday1 = 0;
        let mut datehr1 = 0;
        let mut datemin1 = 0;
        let mut datesec1 = 0;
        let mut filesize_int: i32 = 0;

        let mut bolok = true;

// see if directories have files
        let treemodel1 = ctree_view1_clonecm.get_model();
        if treemodel1 == None {
             messageval_label_clonecm.set_markup("<span color=\"#FF000000\">********* convert Merge: ERROR NOTHING IN DIRECTORY LIST **********</span>");
             bolok = false;
        }

// make sure both directories exist and are not the same
        if bolok {
            if let Some(cur_dir1) = cdirectory1_combobox_clonecm.get_active_text() {
                str_cur_dir1 = format!("{}", cur_dir1);
            } else {
                messageval_label_clonecm.set_markup("<span color=\"#FF000000\">********* convert Merge: ERROR GETTING DIRECTORY IN COMBOBOX **********</span>");
                bolok = false;
            }
        }
// see if filesize exists and is between 4 and 16
        if bolok {
            let filesize_text = cfilesize_entry_clonecm.get_text();
            filesize_int = filesize_text.parse().unwrap_or(-99);
            if filesize_int > 0 {
                if (filesize_int < 4) | (filesize_int > 16) {
                    messageval_label_clonecm.set_markup("<span color=\"#FF000000\">********* convert Merge: Invalid file length. Must be between 4 and 16 **********</span>");
                    bolok = false;
                }
            } else if filesize_int == -99 {
                messageval_label_clonecm.set_markup("<span color=\"#FF000000\">********* convert Merge: Files length is not an integer **********</span>");
                bolok = false;
            } else {
                messageval_label_clonecm.set_markup("<span color=\"#FF000000\">********* convert Merge: File length is not positive integer **********</span>");
                bolok = false;
            }
        }
// validate date mod 
        if bolok {
            let datemod1_text = cdatemod1_entry_clonecm.get_text();
            let datemod1_textx: &String = &format!("{}", datemod1_text);
            let date1ar1: Vec<&str> = datemod1_textx[0..].split(":").collect();
            let lendat1 = date1ar1.len();
            if (lendat1 > 6) | (lendat1 < 6) {
                bolok = false;
            } else {
                for indl in 0..lendat1 {
                     let date_int: i32 = date1ar1[indl].clone().parse().unwrap_or(-9999);
                     if date_int == -9999 {
                         bolok = false;
                     } else {
                         match indl {
                            0 => dateyr1 = date_int as i64,
                            1 => datemo1 = date_int as i64,
                            2 => dateday1 = date_int as i64,
                            3 => datehr1 = date_int as i64,
                            4 => datemin1 = date_int as i64,
                            5 => datesec1 = date_int as i64,
                            _ => bolok = false,
                         }
                     }
                }
            }
            if !bolok {
                messageval_label_clonecm.set_markup("<span color=\"#FF000000\">********* convert Merge: Date Mod 1 is not formatted correctly **********</span>");
            }
        }
        if bolok {
            let current_dir = PathBuf::from(&str_cur_dir1);
            let (errcd1, errstr1, newvector1) = get_strvector(current_dir, 1, filesize_int, false, dateyr1, dateday1, datemo1, datehr1, datemin1, datesec1);
            let mut newvectormut = newvector1;
            let mut chgseq2 = false;
            if errcd1 != 0 {
                messageval_label_clonecm.set_markup(&errstr1);
                bolok = false;
            } else {
                newvectormut.sort();
                let newvectormutlen = newvectormut.len();
                let newtoi = newvectormutlen as i32 ;
                if newtoi > 1 {
                    let mut chgx = true;
                    while chgx {
                       let mut listitems: Vec<String> = Vec::new();
                       chgx = false;
                       for indexi in 1..newtoi {
                            let strinput1split: Vec<&str> = newvectormut[(indexi - 1) as usize].split("|").collect();
                            let strinputsplit: Vec<&str> = newvectormut[indexi as usize].split("|").collect();
                            let mut file_prefixdate1 = format!(" ");
                            let mut file_prefixdate2 = format!(" ");
                            if chgseq2 {
                                chgseq2 = false; 
                                let prefix1: String = strinput1split[0][0..19].parse().unwrap();
                                let mut seq2_int: i32 = strinput1split[0][20..].parse().unwrap_or(-9999);
                                if seq2_int == -9999 {
                                    bolok = false;
                                    chgx = false;
                                    messageval_label_clonecm.set_markup("<span color=\"#FF000000\">********* convert Merge: seq number not numeric **********</span>");
                                    break;
                                } else {
                                    if seq2_int < 999 {
                                        seq2_int = seq2_int + 1;
                                        chgx = true;
                                    }
                                    file_prefixdate1 = format!("{}_{:03}", prefix1, seq2_int);
                                }
                            } else {
                                file_prefixdate1 = format!("{}", strinput1split[0]);
                            }
                            file_prefixdate2 = format!("{}", strinputsplit[0]);
                            if file_prefixdate1 == file_prefixdate2 {
                                chgseq2 = true;
                                let prefix1: String = strinput1split[0][0..19].parse().unwrap();
                                let mut seq1_int: i32 = strinput1split[0][20..].parse().unwrap_or(-9999);
                                if seq1_int == -9999 {
                                    bolok = false;
                                    chgx = false;
                                    messageval_label_clonecm.set_markup("<span color=\"#FF000000\">********* convert Merge: seq number not numeric **********</span>");
                                    break;
                                } else {
                                    if seq1_int > 0 {
                                        seq1_int = seq1_int - 1;
                                        chgx = true;
                                    }
                                }    
                                file_prefixdate1 = format!("{}_{:03}", prefix1, seq1_int);
                            }
                            let stroutput = format!("{}|{}|{}|{}|{}", file_prefixdate1, strinput1split[1], strinput1split[2], strinput1split[3], strinput1split[4]);
                            listitems.push(stroutput);
                       }

                       let current_dira = PathBuf::from(&str_cur_dir1);
                       let (errcda, errstra, newvectora) = get_strvector(current_dira, 1, filesize_int, false, dateyr1, dateday1, datemo1, datehr1, datemin1, datesec1);
                       let mut newvectormuta = newvectora;
                       if errcda != 0 {
                           messageval_label_clonecm.set_markup(&errstra);
                           bolok = false;
                       } else {
                           newvectormuta.sort();
                           let strinputxsplit: Vec<&str> = newvectormuta[(newtoi - 1) as usize].split("|").collect();
                           let mut file_prefixdatex = format!(" ");
                           if chgseq2 {
                               chgseq2 = false; 
                               let prefixx: String = strinputxsplit[0][0..19].parse().unwrap();
                               let mut seqx_int: i32 = strinputxsplit[0][20..].parse().unwrap_or(-9999);
                               if seqx_int == -9999 {
                                   bolok = false;
                                   chgx = false;
                                   messageval_label_clonecm.set_markup("<span color=\"#FF000000\">********* convert Merge: seq number not numeric **********</span>");
                                   break;
                               } else {
                                   if seqx_int < 999 {
                                       seqx_int = seqx_int + 1;
                                       chgx = true;
                                   }
                                   file_prefixdatex = format!("{}_{:03}", prefixx, seqx_int);
                               }
                           } else {
                               file_prefixdatex = format!("{}", strinputxsplit[0]);
                           }
                           let stroutputx = format!("{}|{}|{}|{}|{}", file_prefixdatex, strinputxsplit[1], strinputxsplit[2], strinputxsplit[3], strinputxsplit[4]);
                           listitems.push(stroutputx);
                           newvectormut = listitems;
                           newvectormut.sort();
                       }
                    }
                }
            }
            if bolok {
                let newvectormutlen = newvectormut.len();
                let newtoi = newvectormutlen as i32 ;
                let new_model = ListStore::new(&[String::static_type(), String::static_type(), String::static_type(), String::static_type(), String::static_type(), String::static_type()]);
                for indexi in 0..newtoi {
                     let strinputx = &newvectormut[indexi as usize];
                     let strinputsplitx: Vec<&str>  = strinputx.split("|").collect();
                     new_model.insert_with_values(None,
                          &[FIRST_COL as u32, SECOND_COL as u32, THIRD_COL as u32, FORTH_COL as u32, FIFTH_COL as u32,],
                          &[&strinputsplitx[1], &strinputsplitx[2], &strinputsplitx[0], &strinputsplitx[3], &strinputsplitx[4]]);
                }
                ctree_view3_clonecm.set_model(Some(&new_model));
                let messvalx = format!("convert merge merged {} files", newtoi);
                messageval_label_clonecm.set_text(&messvalx);
            }
        } 
    });
//----------------- convert merge button end -----------------------------------

//----------------- convert copy button start -----------------------------------
//    let cdirectory1_combobox_weakcc = cdirectory1_combobox.downgrade();
//    let cdirectory_o_combobox_weakcc = cdirectory_o_combobox.downgrade();
//    let ctree_view3_weakcc = ctree_view3.downgrade();
//    let messageval_label_weakcc = messageval_label.downgrade();
//    let progress_progressbar_weakcc = progress_progressbar.downgrade();
    let cdirectory1_combobox_clonecc = cdirectory1_combobox.clone();
    let cdirectory_o_combobox_clonecc = cdirectory_o_combobox.clone();
    let ctree_view3_clonecc = ctree_view3.clone();
    let messageval_label_clonecc = messageval_label.clone();
    let progress_progressbar_clonecc = progress_progressbar.clone();
    ccopy_button.connect_clicked( move|_| {

//    ccopy_button.connect_clicked(clone!(@weak cdirectory1_combobox, @weak ctree_view3, @weak progress_progressbar, @weak messageval_label  => move|_| {
//        let cdirectory1_combobox = upgrade_weak!(cdirectory1_combobox_weakcc);
//        let cdirectory_o_combobox = upgrade_weak!(cdirectory_o_combobox_weakcc);
//        let ctree_view3 = upgrade_weak!(ctree_view3_weakcc);
//        let progress_progressbar = upgrade_weak!(progress_progressbar_weakcc);
//        let messageval_label = upgrade_weak!(messageval_label_weakcc);

        let mut bolok = true;
        let mut str_cur_dir1 = format!(" ");
        let mut str_cur_dir_o = format!(" ");
        let mut str_cur_dirfrom = String::new();

// check if both directories exist and they are not equal
        if bolok {
            if let Some(cur_dir1) = cdirectory1_combobox_clonecc.get_active_text() {
                str_cur_dir1 = cur_dir1.to_string();
            } else {
                messageval_label_clonecc.set_markup("<span color=\"#FF000000\">********* convert COPY: ERROR GETTING FROM DIRECTORY IN COMBOBOX **********</span>");
                bolok = false;
            }
        }

// check if outdirectory has files (must not have files)
        if bolok {
            if let Some(cur_dir_o) = cdirectory_o_combobox_clonecc.get_active_text() {
                str_cur_dir_o = cur_dir_o.to_string();
                for entry1 in fs::read_dir(&str_cur_dir_o).unwrap() {
                     let entry = entry1.unwrap();
                     if let Ok(metadata) = entry.metadata() {
                         if let Ok(_file_name) = entry.file_name().into_string() {
                             if metadata.is_file() {
                                 messageval_label.set_markup("<span color=\"#FF000000\">********* convert COPY: OUTPUT DIRECTORY HAS FILES IN IT **********</span>");
                                 bolok = false;
                             }
                         }
                     }
                }
            } else {
                messageval_label_clonecc.set_markup("<span color=\"#FF000000\">********* convert COPY: ERROR GETTING OUT DIRECTORY IN COMBOBOX  **********</span>");
                bolok = false;
           }
        }
// check if merge files and if so process
        if bolok {
            let view3model = ctree_view3_clonecc.get_model();
            if view3model == None {
                messageval_label_clonecc.set_markup("<span color=\"#FF000000\">********* convert Copy: ERROR NOTHING IN MERGE LIST **********</span>");
            } else {
                progress_progressbar_clonecc.set_fraction(0.0);
//                while gtk::events_pending() {
//                   gtk::main_iteration_do(true);
//                }
                let view3modeluw = view3model.unwrap();
                let mut valid = true;
                let validval = view3modeluw.get_iter_first().unwrap();
                let mut numrow = 0;
                let numchildren = view3modeluw.iter_n_children(None);
                let mut numprocess = 0;
                while valid {
                      str_cur_dirfrom = str_cur_dir1.clone();
                      let filefromval = view3modeluw.get_value(&validval,1).get::<String>();
                      let filefromstr = format!("{:?}", filefromval);
                      let filefromln = filefromstr.len();
                      let filefromend = filefromln - 3;
                      let filefromstart = 9;
                      let filefromx = filefromstr.get(filefromstart..filefromend).unwrap();
                      let fullfrom = str_cur_dirfrom.clone() + "/" + filefromx;

                      let filepreval = view3modeluw.get_value(&validval,2).get::<String>();
                      let fileprestr = format!("{:?}", filepreval);
                      let filepreln = fileprestr.len();
                      let filepreend = filepreln - 3;
                      let fileprestart = 9;
                      let fileprex = fileprestr.get(fileprestart..filepreend).unwrap();

                      let filetoval = view3modeluw.get_value(&validval,3).get::<String>();
                      let filetostr = format!("{:?}", filetoval);
                      let filetoln = filetostr.len();
                      let filetoend = filetoln - 3;
                      let filetostart = 9;
                      let filetox = filetostr.get(filetostart..filetoend).unwrap();

                      let fullto = str_cur_dir_o.clone() + "/" + fileprex + "_" + filetox;
                      valid = view3modeluw.iter_next(&validval);
//                      println!("copy action of: cp -p {} {}", fullfrom, fullto);
                      if valid & (numprocess < 4) {
                          Command::new("cp")
                                  .arg("-p")
                                  .arg(&fullfrom)
                                  .arg(&fullto)
                                  .spawn()
                                  .expect("failed to execute process");
                          numprocess = numprocess + 1;
                      } else {
                          let _output = Command::new("cp")
                                                .arg("-p")
                                                .arg(&fullfrom)
                                                .arg(&fullto)
                                                .output()
                                                .expect("failed to execute process");
                          numprocess = 0;
                     }
                     numrow = numrow + 1;
                     let progressfr: f64 = numrow as f64 / numchildren as f64;
                     progress_progressbar_clonecc.set_fraction(progressfr);
//                     while gtk::events_pending() {
//                          gtk::main_iteration_do(true);
//                     }
                }
                let messvalx = format!("convert copy copied {} files", numchildren);
                messageval_label_clonecc.set_text(&messvalx);
            }
        }
    });

//----------------- convert copy button end -----------------------------------

//-------------------- connects end

//    gtk::main();
}

