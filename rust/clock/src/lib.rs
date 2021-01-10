/// Elegant Version that uses euklidic division to get positive remainders, even when given negative numbers! (sth. that does not work so easy with simple divison and modulo)
// originally seen in tylerstonge's solution: https://exercism.io/tracks/rust/exercises/clock/solutions/2e04977f696f4ae0985b952abbc8833a
/// Implementing fmt::Display Trait. ToString trait is automatically implemented for any type which implements the Display trait (this means you get to_string() function for free.)
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            // https://doc.rust-lang.org/std/primitive.i32.html#method.div_euclid
            // The euclidic divison produces always positive remainder, which is perfect here, since negative minutes can be safely
            // - devided by 60 resulting in -hours(div_euclid) +minutes(rem_euclid) or you can
            // - divide by 24 to get -days(not used) +hours(rem_euclid)
            hours: (hours + minutes.div_euclid(60)).rem_euclid(24),
            minutes: minutes.rem_euclid(60),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    /* to_string() is the required method for trait std::string::ToString
    ToString trait is automatically implemented for any type which implements the Display trait. As such, ToString shouldn't be implemented directly: Display should be implemented instead, and you get the ToString implementation for free.
    see: https://doc.rust-lang.org/std/string/trait.ToString.html */
    // Therefore this code is now obsolete: */
    // pub fn to_string(&self) -> String {
    //     // https://doc.rust-lang.org/std/fmt/#sign0
    //     format!("{:02}:{:02}", self.hours, self.minutes)
    // }
}

impl fmt::Display for Clock {
    // https://doc.rust-lang.org/std/fmt/trait.Display.html
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.

        // since write! macro uses internally format_args! macro, you can use formatting as described here:
        // https://doc.rust-lang.org/std/fmt/#sign0
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
