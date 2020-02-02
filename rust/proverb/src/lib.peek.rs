// using a Peekable iterator
pub fn build_proverb(list: Vec<&str>) -> String {
    let mut proverb = String::new();

    let mut iter = list.iter().peekable();
    loop {
        let item = match iter.next() {
            Some(item) => item,
            None => break,
        };

        let next_item: &str;
        {
            next_item = match iter.peek() {
                Some(next_item) => next_item,
                None => break,
            };
        }
        proverb.push_str(format!("For want of a {} the {} was lost.\n", item, next_item).as_str());
    }
    if let Some(item) = list.get(0) {
        proverb.push_str(format!("And all for the want of a {}.", item).as_str());
    }

    proverb
}
