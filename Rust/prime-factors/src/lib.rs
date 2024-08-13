pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];
    let mut num = n;
    let mut factor = 2;

    while num > 1 {
        while num % factor == 0 {
            factors.push(factor);
            num /= factor;
        }
        factor += 1;
    }

    factors
}
