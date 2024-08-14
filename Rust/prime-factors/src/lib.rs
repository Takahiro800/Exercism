pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];
    let mut num: u64 = n;
    let mut candidate = 2;

    while num > 1 {
        while num % candidate == 0 {
            factors.push(candidate);
            num /= candidate;
        }

        candidate += 1;
    }

    factors
}
