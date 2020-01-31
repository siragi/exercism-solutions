//from https://exercism.io/tracks/rust/exercises/beer-song/solutions/a060bf95777a4545ba19545c0608bc95
/// Returns the string "n bottles" with pluralization
fn get_bottles(n: u32) -> String {
    match n {
        0 => String::from("no more bottles"),
        1 => String::from("1 bottle"),
        _ => format!("{} bottles", n),
    }
}

/// Creates a string for the nth verse of the beer song
pub fn verse(n: u32) -> String {
    // Starting with 0 bottles will end with 99 bottles
    let ending_bottles = get_bottles(n.checked_sub(1).unwrap_or(99));
    let starting_bottles = get_bottles(n);

    let first_line = match n {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer."),
        _ => format!("{0} of beer on the wall, {0} of beer.", starting_bottles),
    };

    let action = match n {
        0 => String::from("Go to the store and buy some more"),
        1 => String::from("Take it down and pass it around"),
        _ => String::from("Take one down and pass it around"),
    };

    let second_line = format!("{}, {} of beer on the wall.", action, ending_bottles);

    format!("{}\n{}\n", first_line, second_line)
}

/// Returns the entire beer song lyrics from `start` to `end`
pub fn sing(start: u32, end: u32) -> String {
    // Build a vector of verses, then join them with newlines
    (end..=start)
        .rev()
        .map(|verse_number| verse(verse_number))
        .collect::<Vec<_>>()
        .join("\n")
}
