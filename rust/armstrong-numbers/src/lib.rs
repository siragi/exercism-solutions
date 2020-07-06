// https://exercism.io/tracks/rust/exercises/armstrong-numbers/solutions/79d4f3931f6047de9d9642b57f7c5337
// wezm's solution
pub fn is_armstrong_number(num: u32) -> bool {
    let digits = ((num as f64).log10() + 1.).floor() as u32;
    (0..digits)
        .map(|i| (num / 10u32.pow(i) % 10).pow(digits))
        .sum::<u32>()
        == num
}
