/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // Before doing anything I will strip all the whitespace off the str
    let cleaned_code: String = code.chars().filter(|c| !c.is_whitespace()).collect();
    // If len is less than 2 we know that is automatically invalid
    if !cleaned_code.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    if cleaned_code.len() < 2 {
        return false;
    }

    // Reverse the string so that we can iterate over the entries
    let sum_num = cleaned_code
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let digit = c.to_digit(10).unwrap();
            if i % 2 == 1 {
                let doubled = digit * 2;
                if doubled > 9 { doubled - 9 } else { doubled }
            } else {
                digit
            }
        })
        .sum::<u32>();
    sum_num % 10 == 0
}
