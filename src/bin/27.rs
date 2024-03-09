fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i*i <= n {
        if n % i == 0 {
            return false
        }
        i += 1;
    }
    return true;
}

fn count_primes(a: i32, b: i32) -> u32 {
    let mut n = 0;
    while is_prime(n*n + a * n + b) {
        n += 1;
    }

    return n as u32;
}

fn main() {
    let mut max = 0;
    let mut ans = 0;
    for a in -999..1000 {
        for b in -1000..=1000 {
            let c = count_primes(a, b);
            if c > max {
                max = c;
                ans = a * b;
            }
        }
    }
    println!("{}", ans);
}