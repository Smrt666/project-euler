fn is_leap_year(year: i32) -> bool {
    if year % 400 == 0 {
        true
    } else if year % 100 == 0 {
        false
    } else if year % 4 == 0 {
        true
    } else {
        false
    }
}


fn main() {
    let mut year = 1900;
    let mut month = 1;
    let mut day = 1;
    let mut weekday = 0;

    let mut count = 0;
    while year != 2000 || month != 12 || day != 31 {
        // print!("Year = {} month = {} day = {}, weekday = {weekday}\n", year, month, day);

        if day == 1 && weekday == 6 && year > 1900{
            count += 1;
        }

        day += 1;
        weekday = (weekday + 1) % 7;
        if is_leap_year(year) && month == 2 && day == 30 {
            month += 1;
            day = 1;
        }
        if !is_leap_year(year) && month == 2 && day == 29 {
            month += 1;
            day = 1;
        }

        if (month - 1 - (month - 1) / 7) % 2 == 0 {
            if day == 32 {
                month += 1;
                day = 1;
            }
        } else {
            if day == 31 {
                month += 1;
                day = 1;
            }
        }

        if month == 13 {
            year += 1;
            month = 1;
        }
    }

    println!("Count = {}", count);  // not 173
}