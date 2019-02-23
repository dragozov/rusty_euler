use super::prime_numbers;

pub fn answer() -> u64 {
    let max = 2_000_000;
    let mut primes: Vec<u64> = Vec::new();
    let mut p = 3;
    let mut sum = 5;
    primes.push(2);
    primes.push(3);
    while p < max {
        p = p + 2;
        if prime_numbers::is_prime(p, &primes) {
            primes.push(p);
            sum += p
        }
    }
    sum
}
