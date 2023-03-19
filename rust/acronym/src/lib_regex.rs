// Adapted from godtopus's solution: https://exercism.io/tracks/rust/exercises/acronym/solutions/95aa8536d16746f09807e63929984b2a#solution-comment-171809
// Regex from ryanplusplus: https://exercism.io/tracks/rust/exercises/acronym/solutions/28adbfcc9fc342cc9bad06bd4a42694c
// Info: https://docs.rs/regex/1.4.3/regex/ (x* -> zero or more of x, y+ -> one or more of y)
use regex::Regex;

pub fn abbreviate(sequence: &str) -> String {
    let mut acronym = String::new();
    let re = Regex::new(r"[A-Z]*[a-z']+|[A-Z]+").unwrap();

    // Regex::captures returns Option<Captures>, First Capture Element holds the full match as str: CamelCase -> Camel, next loop generates -> Case, and str can be sliced, when given a range (so you take the first letter with capture[0][..1] ).
    for capture in re.captures_iter(sequence) {
        acronym.push_str(&capture[0][..1]); // https://docs.rs/regex/1.4.3/regex/struct.Captures.html
    }

    acronym.to_uppercase()
}
