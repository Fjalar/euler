fn main() {
    let mut year = 1900;
    let mut month = 0;
    let mut weekday = 0;

    let mut total_sundays = 0;

    while year < 2001 {
        if weekday == 0 && year >= 1901 {
            total_sundays += 1;
            // println!("1st of {} {year} was a sunday", month_name(month));
        }
        weekday += day_this_month(month, year);
        weekday %= 7;

        month += 1;
        if month > 11 {
            year += 1;
            month = 0;
        }
    }

    println!("{total_sundays}");
}

// zero-indexed month number
fn day_this_month(month_num: u32, year: u32) -> u32 {
    match month_num {
        0 => 31, // january
        1 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        } // february
        2 => 31, // march
        3 => 30, // april
        4 => 31, // may
        5 => 30, // june
        6 => 31, // july
        7 => 31, // august
        8 => 30, // september
        9 => 31, // october
        10 => 30, // november
        11 => 31, // december
        _ => panic!(),
    }
}

#[allow(dead_code)]
fn month_name(month_num: u32) -> &'static str {
    match month_num {
        0 => "january",
        1 => "february",
        2 => "march",
        3 => "april",
        4 => "may",
        5 => "june",
        6 => "july",
        7 => "august",
        8 => "september",
        9 => "october",
        10 => "november",
        11 => "december",
        _ => panic!(),
    }
}

fn is_leap_year(year: u32) -> bool {
    year.is_multiple_of(4) && !year.is_multiple_of(400)
}
