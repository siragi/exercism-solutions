pub fn verse(n: u32) -> String {
    // unimplemented!("emit verse {}", n)
    let mut a_verse = String::new();

    match n {
        0 => a_verse.push_str(
            "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n",
        ),
        1 => a_verse.push_str(
            "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"
        ),
        n @ 2 => a_verse.push_str(&format!(
            "{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottle of beer on the wall.\n",
            n,
            n - 1
        )),
        n => a_verse.push_str(&format!(
            "{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n",
            n,
            n - 1
        )),
    }
    a_verse
}

pub fn sing(start: u32, end: u32) -> String {
    //unimplemented!("sing verses {} to {}, inclusive", start, end)
    let beers = end..=start;
    let mut song = String::new();
    //beers.for_each(|beer| song.push_str(&format!("{}\n\n", verse(beer))));
    for beer in beers {
        let songline = verse(beer);
        song.insert_str(0, &format!("{}\n", songline));
    }
    song = song.trim_end().to_string();
    song.push('\n');
    song
}
