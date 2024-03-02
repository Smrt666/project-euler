fn largest_prime_factor(n: u64) -> u64 {
    let mut n = n;
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            n /= i;
        } else {
            i += 1;
        }
    }
    return n;
}

fn main() {
    println!("{}", largest_prime_factor(13195));
    println!("{}", largest_prime_factor(600851475143));
}