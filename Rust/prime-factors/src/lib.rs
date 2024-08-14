pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];
    let mut num: u64 = n;
    let mut canditate: u64 = 2;

    while num > 1 {
        while num % canditate == 0 {
            factors.push(canditate);
            num /= canditate;
        }

        canditate += 1;
    }

    factors
}
