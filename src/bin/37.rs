fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn is_t_prime(n: u32) -> bool {
    if n < 10 {
        return false;
    }
    let mut p = 10;
    while p < n {
        if !is_prime(n % p) || !is_prime(n / p){
            return false;
        }
        p *= 10;
    }
    is_prime(n)
}

fn main() {
    let mut count = 0;
    let mut sum = 0;
    let mut i = 11;
    while count < 11 {
        if is_t_prime(i) {
            count += 1;
            sum += i;
        }
        i += 2;
    }
    println!("{}", sum);
}