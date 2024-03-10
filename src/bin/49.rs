fn digitcount(n: u32) -> Vec<u32> {
    let mut digits = vec![0; 10];
    let mut n = n;
    while n > 0 {
        digits[(n % 10) as usize] += 1;
        n /= 10;
    }
    digits
}

fn main() {
    let mut sieve = [true; 100000];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..sieve.len() {
        if sieve[i] {
            for j in (i * i..sieve.len()).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    for i in 1000..10000 {
        if !sieve[i] {
            continue;
        }
        for step in 1..(10000 - i) {
            let j = i + step;
            let k = j + step;
            if j >= 10000 || k >= 10000 {
                break;
            }
            if sieve[j] && sieve[k] {
                let di = digitcount(i as u32);
                let dj = digitcount(j as u32);
                let dk = digitcount(k as u32);
                if di == dj && dj == dk {
                    println!("{}{}{}", i, j, k);
                }
            }
        }
    }
}