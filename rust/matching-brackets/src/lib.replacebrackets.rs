/// My solution replaces inner pairs of "()","{}" and "[]" with Nullstrings

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut is_balanced = false;

    // only relevant Chars shall be analyzed and collected from given string
    let mut filtered: String = string
        .chars()
        .filter(|c| ['(', ')', '[', ']', '{', '}'].contains(c))
        .collect();

    // Let's get the obvious out of the way
    if filtered.len() == 0 {
        // No brackets means balanced (without brackets)
        is_balanced = true;
    } else if filtered.len() % 2 != 0 {
        // Return false, when filtered String cannot possibly form pairs of brackets
        is_balanced = false;
    } else {
        // in maximum n_pairs
        let n_pairs = filtered.len() / 2;

        for _ in 0..n_pairs {
            // replace pairs by nothing
            filtered = filtered.replace("()", "");
            filtered = filtered.replace("{}", "");
            filtered = filtered.replace("[]", "");
            // resulting in absolutely nothing, when all pairs are ok. So we return true.
            if filtered.len() == 0 {
                is_balanced = true;
                break;
            }
        }
    }

    is_balanced
}
