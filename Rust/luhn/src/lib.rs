/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.replace(" ", "");

    if code.len() <= 1 {
        return false;
    }

    if !code.chars().all(|c| c.is_digit(10)) {
        return false;
    }

    let sum: u32 = code
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let mut digit = c.to_digit(10).unwrap();

            if i % 2 == 1 {
                digit *= 2;
                if digit > 9 {
                    digit -= 9;
                }
            }

            digit
        })
        .sum();

    sum % 10 == 0
}
