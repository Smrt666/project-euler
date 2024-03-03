fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}

fn next_prime(n: u32) -> u32 {
    let mut n = n + 1;
    while !is_prime(n) {
        n += 1;
    }
    return n;
}


fn main() {
    let mut n = 2;
    for _ in 0..10000 {
        n = next_prime(n);
    }
    println!("{}", n);
}