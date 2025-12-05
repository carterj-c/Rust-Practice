pub fn square_of_sum(n: u32) -> u32 {
    // let mut rolling_sum = 0;
    //
    // for integer in 1..=n {
    //     rolling_sum += integer;
    // }
    // rolling_sum.pow(2)
    (1..=n).sum::<u32>().pow(2) // Hilarious how tiny you can make this, I did the verbose
    // version before researching other ways!
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|i| i.pow(2)).sum()
}

pub fn difference(n: u32) -> u32 {
    sum_of_squares(n).abs_diff(square_of_sum(n))
}
