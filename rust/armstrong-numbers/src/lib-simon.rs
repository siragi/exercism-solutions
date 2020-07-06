/// An Armstrong number is a number that is the sum of its own digits each raised to the power of the number of digits.
pub fn is_armstrong_number(num: u32) -> bool {
    // true if 'num' is an armstrong number
    // 153 is an Armstrong number, because: 153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153

    let mut armstrong: u32 = 0;

    // a) Using Strings and Chars (char iterator)
    let num_string = num.to_string();
    let mut num_chars = num_string.chars();
    let digits = num_string.chars().count() as u32;

    while let Some(c) = num_chars.next() {
        if let Some(d) = c.to_digit(10) {
            armstrong += d.pow(digits);
        }
    }

    // b) Using the decimal log
    /* let mut num_f32 = num as f32;
    let num_log10 = num_f32.log10();
    let digits = num_log10.floor() as u32 + 1;

    for p in (0..digits).rev() {
        // let x = num / 10_u32.pow(p);
        //armstrong += x.pow(p);
        let d = (num_f32 / 10_f32.powi(p as i32)).trunc();
        num_f32 = num_f32 - (d * 10_f32.powi(p as i32));
        armstrong += (d as u32).pow(digits);
    } */

    armstrong == num
}
