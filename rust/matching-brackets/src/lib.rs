/// Most elegant version checks if given Char
///   - c opens a bracket and therefore fills the Vec with needed closing brackets OR
///   - c equals the last needed closing bracket (which consumes the last needed bracket)
//
// Originally cN3rd's solution (2020): https://exercism.io/tracks/rust/exercises/matching-brackets/solutions/cc4c68f6a0d44303b12401beb7c717e3
// which may be inspired by dbalmain's solution (2018): https://exercism.io/tracks/rust/exercises/matching-brackets/solutions/70687b13c48040a68fcc1539108e1a40
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut closing_brackets_needed = Vec::new();
    for c in string.chars() {
        match c {
            ')' | ']' | '}' if closing_brackets_needed.pop() != Some(c) => {
                // if given char c doesn't match the last (pop does LIFO) needed closing brackets:
                return false;
            }
            '(' => closing_brackets_needed.push(')'),
            '[' => closing_brackets_needed.push(']'),
            '{' => closing_brackets_needed.push('}'),
            _ => (),
        }
    }

    // No more closing brackets needed, means balanced
    closing_brackets_needed.is_empty()
}
