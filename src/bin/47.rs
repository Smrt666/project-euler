fn factors(n: u32) -> u32 {
    let mut factors = 0;
    let mut i = 2;
    let mut n = n;
    while i * i <= n {
        if n % i == 0 {
            factors += 1;
        }
        while n % i == 0 {
            n /= i;
        }
        i += 1;
    }
    if n > 1 {
        factors += 1;
    }
    factors
}

fn main() {
    let mut i = 2;
    let mut count = 0;
    loop {
        if factors(i) == 4 {
            count += 1;
            if count == 4 {
                println!("{}", i - 3);
                break;
            }
        } else {
            count = 0;
        }
        i += 1;
    }
}