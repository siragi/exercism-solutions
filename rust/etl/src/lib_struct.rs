/* Extract-Transform-Load (ETL) is a fancy way of saying, "We have some crufty, legacy data over in this system, and now we need it in this shiny new system over here, so we're going to migrate this." */
use std::collections::BTreeMap;

pub fn transform(source: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut bt = BTreeMap::new();
    for (scores, chars) in source {
        for c in chars {
            bt.insert(c.to_ascii_lowercase(), *scores);
        }
    }
    bt
}
