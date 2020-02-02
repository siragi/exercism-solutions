use std::iter::once;

/// build a proverb "For want of a {shoe} the {nail} was lost" from a given list of words
pub fn build_proverb(list: &[&str]) -> String {
    // Empty List -> Return empty String
    if list.is_empty() {
        return String::new();
    }
    // Verses
    list.windows(2) // create iterator over overlapping subslices of length 2.
        .map(|subslice| {
            format!(
                "For want of a {} the {} was lost.",
                subslice[0], subslice[1]
            )
        }) //  btw.: if the slice is shorter than 2, the Windows iterator returns no values, but continous..
        .chain(once(format!("And all for the want of a {}.", list[0]))) // .. to add a single String
        .collect::<Vec<_>>()
        .join("\n")
}
