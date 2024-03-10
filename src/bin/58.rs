fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n < 2 || n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn main() {
    let mut i = 1;
    let mut step = 2;
    let mut primes: u32 = 0;
    let mut total: u32 = 1;
    loop {
        if step % 10000 == 0 {
            println!("step={} : {} / {}", step, primes, total);
        }
        for _ in 0..4 {
            i += step;
            if is_prime(i) {
                primes += 1;
            }
            total += 1;
        }
        step += 2;
        if primes * 10 < total {
            println!("Result: {}", step - 1);  // not 866249
            return;
        }
    }
}