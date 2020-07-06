// a) Without bonus task of reversing uüu
/* pub fn reverse(input: &str) -> String {
    // unimplemented!("Write a function to reverse {}", input);
    //input.chars().rev().collect::<String>()
    // Turbofish is not necessary since the fn determines the Return Type
    input.chars().rev().collect()
    // OR:
    //input.chars().rfold("".to_string(), |acc, ch| acc + &ch.to_string())
    // OR:
    //input.rsplit("").collect()
} */

// b) Solves bonus task of reversing uüu (does Segmentation into Grapheme Clusters)
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // unimplemented!("Write a function to reverse {}", input);
    //UnicodeSegmentation::graphemes(input, true).rev().collect()
    // OR
    input.graphemes(true).rev().collect()
}
