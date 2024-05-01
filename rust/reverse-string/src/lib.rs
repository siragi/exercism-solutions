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
// use unicode_segmentation::UnicodeSegmentation;

// pub fn reverse(input: &str) -> String {
//     // unimplemented!("Write a function to reverse {}", input);
//     //UnicodeSegmentation::graphemes(input, true).rev().collect()
//     // OR
//     input
// .graphemes(true) // Split the string into an Iterator of &strs, where each element is an
// extended grapheme cluster.
// .rev() // Reverse the order of the grapheme iterator.
// .collect()
// }

//c) reverse chars using swap on mutable vector
pub fn reverse(input: &str) -> String {
    let s = input.to_string();
    if input.is_empty() {
        return s;
    }

    let mut v: Vec<char> = s.chars().collect();

    let items = v.len();
    // Iterate over half the items
    for i in 0..items / 2 {
        v.swap(i, items - i - 1);
    }
    v.iter().cloned().collect()
}

// not working - :-)
// #[allow(unused)]
// use std::ptr::{self, swap};
// #[allow(unused)]
// pub fn reverse(input: &str) -> String {
//     if input.len() < 2 {
//         return input.to_string();
//     }

//     let len = input.len();
//     let mut s = String::from(input);
//     let mut ptr = s.as_mut_ptr();
//     // let ptr = &mut s as *mut &str;
//     for i in 0..=(len - 1) / 2 {
//         // for i in 0..2 {

//         unsafe {
//             dbg!("dbg!: {}: {}", i, *ptr.add(i));
//             swap(ptr.wrapping_add(i), ptr.wrapping_add(len - 1 - i));
//             // swap(pa, pb);
//         }
//     }
//     s
// }
