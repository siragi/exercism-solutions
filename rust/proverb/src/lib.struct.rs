/// build a proverb "For want of a {shoe} the {nail} was lost" from a given list of items: Here the list contained [shoe, nail].
pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();
    // Empty List -> Return empty String
    if list.is_empty() {
        return proverb;
    }
    // Two or more words in the list -> "For want of a ..."
    //      from i = 1 means : there is list[0] and [1] to start with.
    for i in 1..list.len() {
        proverb.push_str(&format!(
            "For want of a {} the {} was lost.\n",
            list[i - 1],
            list[i],
        ))
    }
    // Ending verse
    proverb.push_str(&format!("And all for the want of a {}.", list[0]));
    proverb
}
