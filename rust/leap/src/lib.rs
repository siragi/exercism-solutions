pub fn is_leap_year(year: u64) -> bool {
    // "true if {} is a leap year", year)
    let mut is_leap = false;
    if year % 4 == 0 {
        if year >= 100 && year % 100 == 0 {
            if year >= 400 && year % 400 == 0 {
                println!("div 4 and 100 ok, div 400 ok too -> leap year");
                is_leap = true;
            } else {
                println!("div 4 and 100 ok, but not div 400 -> no leap year");
            }
        } else {
            is_leap = true;
        }
    } else {
        println!("div 4 nok");
    }
    is_leap
}
