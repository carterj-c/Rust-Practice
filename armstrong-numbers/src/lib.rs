pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = Vec::new();
    let mut num_stored = num;

    while num_stored > 0 {
        digits.push(num_stored % 10);
        num_stored /= 10;
    }

    let num_len = digits.len() as u32;
    let mut armstrong_num = 0;

    for digit in digits {
        armstrong_num += digit.pow(num_len);
    }

    // Check the armstrong_num
    armstrong_num == num
}
