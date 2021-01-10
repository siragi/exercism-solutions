/// This elegant version checks if given Char
///   - c opens a bracket and therefore fills the Vec with opening brackets OR
///   - c closes the last recorded opening bracket (which consumes the last opening bracket)
//
// Originally Hwatwasthat's solution (2019): https://exercism.io/tracks/rust/exercises/matching-brackets/solutions/cc4c68f6a0d44303b12401beb7c717e3
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets: Vec<char> = vec![];
    for c in string.chars() {
        match c {
            '('  | '{' | '[' => brackets.push(c),
            '}' => if brackets.pop() != Some('{') { return false},
            ')' => if brackets.pop() != Some('(') { return false},
            ']' => if brackets.pop() != Some('[') { return false},
            _ => (),
        }
    }
    brackets.is_empty()
}