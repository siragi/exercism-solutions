/// Not so elegant Version that treats negative minutes/hours differently (sth. that does not work so easy using simple divison and modulo)
/// Implementing fmt::Display Trait. ToString trait is automatically implemented for any type which implements the Display trait (this means you get to_string() function for free.)
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    // https://doc.rust-lang.org/std/fmt/trait.Display.html
    // fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // since write! macro uses internally format_args! macro, you can use formatting as described here:
        // https://doc.rust-lang.org/std/fmt/#sign0
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // Construct a new Clock from {} hours and {} minutes
        let mut new_clock = Self { hours, minutes };

        let all_minutes = hours * 60 + minutes;
        if all_minutes < 0 {
            let mut hour_basis = 23;
            if all_minutes % 60 == 0 {
                hour_basis = 24;
            }
            new_clock.hours = (hour_basis + (all_minutes / 60 % 24)) % 24;
            new_clock.minutes = (60 + all_minutes % 60) % 60;
        } else {
            new_clock.hours = (24 + (all_minutes / 60)) % 24;
            new_clock.minutes = (60 + all_minutes) % 60;
        }

        new_clock
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // Add {} minutes to existing Clock time, minutes
        Self::new(self.hours, self.minutes + minutes)
    }

    // to_string() is the required method for trait std::string::ToString
    // ToString trait is automatically implemented for any type which implements the Display trait. As such, ToString shouldn't be implemented directly: Display should be implemented instead, and you get the ToString implementation for free.
    // see: https://doc.rust-lang.org/std/string/trait.ToString.html
    /* pub fn to_string(&self) -> String {
        // https://doc.rust-lang.org/std/fmt/#sign0
        format!("{:02}:{:02}", self.hours, self.minutes)
    } */
}
