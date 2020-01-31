pub fn verse(n: u32) -> String {
    // Matches the appropriate verse to available n bottles of beer
    let mut a_verse = String::new();

    match n {
        0 => a_verse.push_str(
            "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n",
        ),
        _ => a_verse.push_str(&format!(
            "{n} bottle{s} of beer on the wall, {n} bottle{s} of beer.\nTake {one} down and pass it around, {n_1} bottle{s_1} of beer on the wall.\n",
            n=n,
            one=match n { 
                1 => "it",
                _ => "one",
            },
            s=match n {
                1 => "",
                _ => "s"
            },
            n_1=match n-1 {
                0 => "no more".to_string(), // type mismatch between &str ...  
                _ => (n-1).to_string(),  // and u32 forced me to use common Type String
            },
            s_1=match n-1 {
                0 => "s",
                1 => "",
                _ => "s",
            },
        )),
    }
    a_verse
}

pub fn sing(start: u32, end: u32) -> String {
    // you get the song, verse by verse (with a trailing new line)

    let beers = (end..=start).rev(); // Ranges can only grow, but they can be reversed afterwards.

    // My original solution:
    //   beers.for_each(|beer| song.push_str(&format!("\n{}", verse(beer))));
    //   song.trim_start().to_string() */

    // Build a vector of verses, then join them with newlines 
    beers
        .map(|beer| verse(beer))
        .collect::<Vec<_>>()
        .join("\n")
}
