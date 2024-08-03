pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let sum: u64 = digits.iter().fold(0_u64, |sum, num| {
        sum + (*num as u64).pow(digits.len() as u32)
    });

    num as u64 == sum
}
