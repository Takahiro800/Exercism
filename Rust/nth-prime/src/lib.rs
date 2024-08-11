pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![];

    (2..)
        .filter(|&num| {
            if primes.iter().all(|&prime| num % prime != 0) {
                primes.push(num);
                true
            } else {
                false
            }
        })
        .nth(n as usize)
        .unwrap()
}
