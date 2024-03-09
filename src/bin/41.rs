fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn is_pandigital(n: u32) -> bool {
    let mut digits = [false; 10];
    let mut n = n;
    while n > 0 {
        let digit = n % 10;
        if digit == 0 || digits[digit as usize] {
            return false;
        }
        digits[digit as usize] = true;
        n /= 10;
    }
    
    let mut f = false;
    for &i in digits.iter().skip(1) {
        if i && f {
            return false;
        }
        if !i {
            f = true;
        }
    }
    true
}

fn main() {
    let mut mx = 0;
    for i in 1..1000000000 {
        if is_pandigital(i) && is_prime(i) {
            mx = i;
        }
    }
    println!("{}", mx);  // not 4231
}