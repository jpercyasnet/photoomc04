extern crate chrono;
use chrono::offset::LocalResult;
use chrono::prelude::*;
use chrono::{Utc};
// gen: datenumother, mbeforebox, datenumto--, bolusenum--, dateto--, dateother

// function called by build_ui
//  Use to find date to insert into directory
// input is datenumother, mbeforebox, datenumto, dateto, dateother and output is error number, error string, bolusenum, datenumtox, datetox
pub fn celldatename_merge (fromfilename: String, datenumother: i32, mbeforebox_check: bool, datenumto: i32, dateto: DateTime<Utc>, dateother:  DateTime<Utc>) -> (u32, String, bool, i32,  DateTime<Utc>) {
    let mut errcode: u32 = 0;
    let mut errstring: String = " ".to_string();
    let mut datenumtox: i32 = 0;
    let bolusenum = false;
    let mut datetox = dateother.clone();
    let mut dateyr: i32 = 0;
    let mut datemo: u32 = 0;
    let mut dateday: u32 = 0;
    let mut datehr: u32 = 0;
    let mut datemin: u32 = 0;
    let mut datesec: u32 = 0;
    let mut datenum = 0;
    let mut datefile = Utc.ymd(2000,1,1).and_hms_milli(1,1,1,0);

// extract date from file name
//    let fileln = fromfilename.len();
//    let fileend = fileln - 2;
//    let filestart = 6;
//    let fromfilenamex = fromfilename.get(filestart..fileend).unwrap();
    let mut baddate1 = 0;
    let mut yyyymmddx: String = " ".to_string();
    let mut hhmmssx: String = " ".to_string();
    println!("cell: fromfilename: {}", fromfilename);
    if fromfilename.len() < 15 {
        baddate1 = 1;
    } else {
// date in name start
        let date1ar2: Vec<&str> = fromfilename.split("_").collect();
        let lendat2 = date1ar2.len();
        if lendat2 < 2 {
            baddate1 = 1;
        } else {
            yyyymmddx = date1ar2[0].clone().parse().unwrap();
            if yyyymmddx.len() != 8 {
//            if date1ar2(0).clone().parse().unwrap().len() != 8 {
                if lendat2 < 3 {
                    baddate1 = 1;
                } else {
                    yyyymmddx = date1ar2[1].clone().parse().unwrap();
                    if yyyymmddx.len() != 8 {
                        baddate1 = 1;
                    } else {
                        hhmmssx = date1ar2[2].clone().parse().unwrap();
                    }
                }
            } else {
                hhmmssx = date1ar2[1].clone().parse().unwrap();
            }
        }
    }
    if baddate1 == 0 {
        if hhmmssx.len() < 6 {
            baddate1 = 1;
        } else {
            dateyr = yyyymmddx.get(0..4).unwrap().parse().unwrap_or(9999);
            if dateyr == 9999 {
                baddate1 = 1;
            } else {
                datemo = yyyymmddx.get(4..6).unwrap().parse().unwrap_or(9999);
                if datemo == 9999 {
                    baddate1 = 1;
                } else {
                    dateday = yyyymmddx.get(6..8).unwrap().parse().unwrap_or(9999);
                    if dateday == 9999 {
                        baddate1 = 1;
                    } else {
                        let datexx = Local.ymd_opt(dateyr, datemo, dateday);
                        if datexx == LocalResult::None {
                            baddate1 = 1; 
                        } else {
                            datehr = hhmmssx.get(0..2).unwrap().parse().unwrap_or(9999);
                            if datehr == 9999 {
                                baddate1 = 1;
                            } else {
                                if datehr > 23 {
                                    baddate1 = 1;
                                } else {
                                    datemin = hhmmssx.get(2..4).unwrap().parse().unwrap_or(9999);
                                    if datemin == 9999 {
                                        baddate1 = 1;
                                    } else {
                                        if datemin > 59 {
                                            baddate1 = 1;
                                        } else {
                                            datesec = hhmmssx.get(4..6).unwrap().parse().unwrap_or(9999);
                                            if datesec == 9999 {
                                                baddate1 = 1;
                                            } else {
                                                if datesec > 59 {
                                                    baddate1 = 1;
                                                }
                                            }
                                        }            
                                    } 
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if baddate1 != 0 {
        errstring = "<span color=\"#FF000000\">********* Merge: selected file name is does not have a valid date in name **********</span>".to_string();
        errcode = 1;
    } else {
        datefile = Utc.ymd(dateyr, datemo, dateday).and_hms_milli(datehr as u32, datemin as u32, datesec as u32, 0);
// no before or after file
        if datenumother > 999 {
            if mbeforebox_check {
                if dateto == datefile {
                    if datenumto < 1 {
                        errstring = "<span color=\"#FF000000\">********* Merge: selected file number is zero can not insert **********</span>".to_string();
                        errcode = 2;
                    } else {
                        if datenumto == 1 {
                            datenum = 0;
                        } else {
                            datenum = datenumto - (datenumto/2);
                        }
                    }                      
                } else {
                    let duration = dateto.signed_duration_since(datefile);
                    if duration.num_seconds() < 0 {
                        errstring = "<span color=\"#FF000000\">********* Merge: selected file date not less than place of insertion **********</span>".to_string();
                        errcode = 3;
                    } else {
                        datenum = 500;
                    }
                }
            } else {
                if dateto == datefile {
                    if datenumto > 998 {
                        errstring = "<span color=\"#FF000000\">********* Merge: selected file number is 999 can not insert **********</span>".to_string();
                        errcode = 4;
                    } else {
                        datenum = datenumto + ((1000 - datenumto)/2);
                    }                      
                } else {
                    let duration = datefile.signed_duration_since(dateto);
                    if duration.num_seconds() < 0 {
                        errstring = "<span color=\"#FF000000\">********* Merge: selected file date not greater than place of insertion **********</span>".to_string();
                        errcode = 5;
                    } else {
                        datenum = 500;
                    }
                }
            }
// have before or after file
        } else {
            if dateto == dateother {
                if datefile != dateto {
                    errstring = "<span color=\"#FF000000\">********* Merge: selected file date will not go into place of insertion 6 **********</span>".to_string();
                    errcode = 6;
                } else {
                    if mbeforebox_check {
                        if (datenumto - datenumother) < 2 {
                            errstring = "<span color=\"#FF000000\">********* Merge: before checked and selected file number and previous file number less than 2 apart **********</span>".to_string();
                            errcode = 7;
                        } else {
                            datenum = datenumto - ((datenumto - datenumother)/2);
                        }
                    } else {
                        if (datenumother - datenumto) < 2 {
                            errstring = "<span color=\"#FF000000\">********* Merge: selected file number and next file number less than 2 apart **********</span>".to_string();
                            errcode = 8;
                        } else {
                            datenum = datenumto + ((datenumother - datenumto)/2);
                        }
                    }
                }  
            } else {
                if mbeforebox_check {
                    if dateother == datefile {
                        if datenumother > 998 {
                            errstring = "<span color=\"#FF000000\">********* Merge: selected before file number is 999 can not insert **********</span>".to_string();
                            errcode = 9;
                        } else {
                            datenum = datenumother + ((1000 - datenumother)/2);
                        }
                    } else {
                        let duration = datefile.signed_duration_since(dateother);
                        if duration.num_seconds() < 0 {
                            errstring = "<span color=\"#FF000000\">********* Merge: selected file date number will not go into place of insertion 10 **********</span>".to_string();
                            errcode = 10;
                        } else {
                            if dateto == datefile {
                                if datenumto < 1 {
                                    errstring = "<span color=\"#FF000000\">********* Merge: selected file number is zero can not insert **********</span>".to_string();
                                    errcode = 11;
                                } else {
                                    if datenumto == 1 {
                                        datenum = 0;
                                    } else {
                                        datenum = datenumto - (datenumto/2);
                                    }
                                }
                            } else {
                                let duration = dateto.signed_duration_since(datefile);
                                if duration.num_seconds() < 0 {
                                    errstring = "<span color=\"#FF000000\">********* Merge: selected file date number will not go into place of insertion 10 **********</span>".to_string();
                                    errcode = 12;
                                } else {
                                    datenum = 500;
                                }
                            } 
                        }
                    }
                } else {
                    if dateto == datefile {
                        if datenumto > 998 {
                            errstring = "<span color=\"#FF000000\">********* Merge: selected before file number is 999 can not insert **********</span>".to_string();
                            errcode = 13;
                        } else {
                            datenum = datenumto + ((1000 - datenumto)/2);
                        }
                    } else {
                        let duration = datefile.signed_duration_since(dateto);
                        if duration.num_seconds() < 0 {
                            errstring = "<span color=\"#FF000000\">********* Merge: selected file date number will not go into place of insertion 14 **********</span>".to_string();
                            errcode = 14;
                        } else {
                            if dateother == datefile {
                                if datenumother < 1 {
                                    errstring = "<span color=\"#FF000000\">********* Merge: selected file number is zero can not insert **********</span>".to_string();
                                    errcode = 15;
                                } else {
                                    if datenumother == 1 {
                                        datenum = 0;
                                    } else {
                                        datenum = datenumother - (datenumother/2);
                                    }
                                }
                            } else {
                                let duration = dateother.signed_duration_since(datefile);
                                if duration.num_seconds() < 0 {
                                    errstring = "<span color=\"#FF000000\">********* Merge: selected file date number will not go into place of insertion 16 **********</span>".to_string();
                                    errcode = 16;
                                } else {
                                    datenum = 500;
                                }
                            } 
                        }
                    }
                }
            }
        }
    }
    if errcode == 0 {
        datenumtox = datenum;
        datetox = datefile;
    }
    (errcode, errstring, bolusenum, datenumtox, datetox)
}
