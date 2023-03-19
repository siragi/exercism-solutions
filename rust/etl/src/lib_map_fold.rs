/* Extract-Transform-Load (ETL) is a fancy way of saying, "We have some crufty, legacy data over in this system, and now we need it in this shiny new system over here, so we're going to migrate this." */
use std::collections::BTreeMap;

pub fn transform(source: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    source
        .iter()
        .map(|(score, chars)| {
            chars
                .iter()
                .map(|char| (char.to_ascii_lowercase(), *score))
                .collect::<BTreeMap<char, i32>>() //
        })
        .fold(BTreeMap::new(), |mut bt: BTreeMap<char, i32>, mut item| {
            bt.append(&mut item);
            bt
        })
    /* Upper folding can be replaced by: */
    // .flatten()
    // .collect()
}
