fn sum_primes(n: i32) -> u64 {
    let mut sieve = vec![true; n as usize + 1];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..=(n as f64).sqrt() as i32 {
        if sieve[i as usize] {
            for j in (i * i..=n).step_by(i as usize) {
                sieve[j as usize] = false;
            }
        }
    }
    return sieve.iter().enumerate().map(|(i, &p)| if p {i as u64} else {0}).sum();
}

fn main() {
    println!("{}", sum_primes(10));
    println!("{}", sum_primes(2_000_000));
}