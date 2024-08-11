pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let len = num_str.len() as u32;

    let sum: u64 = num_str
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|num| (num as u64).pow(len))
        .sum();

    sum == num as u64
}
