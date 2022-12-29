
// function 
pub fn parse_moddate(modtext: String) -> (u32, i64, i64, i64, i64, i64, i64) { 
    
    let mut errcode: u32 = 0;
    let mut dateyr1: i64 = 0;
    let mut datemo1: i64 = 0;
    let mut dateday1: i64 = 0;
    let mut datehr1: i64 = 0;
    let mut datemin1: i64 = 0;
    let mut datesec1: i64 = 0;
    let datemod1_textx: &String = &format!("{}", modtext);
    let date1ar1: Vec<&str> = datemod1_textx[0..].split(":").collect();
    let lendat1 = date1ar1.len();
    if (lendat1 > 6) | (lendat1 < 6) {
        errcode = 1;
    } else {
        for indl in 0..lendat1 {
             let date_int: i32 = date1ar1[indl].clone().parse().unwrap_or(-9999);
             if date_int == -9999 {
                 errcode = 2;
             } else {
                 match indl {
                    0 => dateyr1 = date_int as i64,
                    1 => datemo1 = date_int as i64,
                    2 => dateday1 = date_int as i64,
                    3 => datehr1 = date_int as i64,
                    4 => datemin1 = date_int as i64,
                    5 => datesec1 = date_int as i64,
                    _ => errcode = 3,
                 }
            }
        }
    }
    (errcode, dateyr1, datemo1, dateday1, datehr1, datemin1, datesec1)
}

