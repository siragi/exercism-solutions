/* Very elegant version, adapted from: https://exercism.io/tracks/rust/exercises/etl/solutions/fd172a455487484da3c9786b70a6e0a6
 - uses the all in one Iter function flatmap(), that returns an Iter per input element
 - moves the ownership of every character into the final tuple (consumes chars)
*/

/* Extract-Transform-Load (ETL) is a fancy way of saying, "We have some crufty, legacy data over in this system, and now we need it in this shiny new system over here, so we're going to migrate this." */
use std::collections::BTreeMap;

pub fn transform(source: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    source
        .iter()
        .flat_map(|(&score, chars)| chars.iter().map(move |c| (c.to_ascii_lowercase(), score)))
        .collect()
}
