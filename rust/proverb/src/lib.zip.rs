pub fn build_proverb(list: Vec<&str>) -> String {
    list.iter()
        .zip(list.iter().skip(1))
        .map(|(a, b)| format!("For want of a {} the {} was lost.", a, b))
        .chain(
            list.iter()
                .take(1)
                .map(|a| format!("And all for the want of a {}.", a)),
        )
        .collect::<Vec<_>>()
        .join("\n")
}
