use itertools::Itertools;

fn factors(n: usize, sieve: &[usize]) -> Vec<usize> {
    if n <= 1 {
        panic!("n should be greater than 1");
    }

    let mut res = vec![];
    let mut n = n;
    while n > 1 {
        let p = sieve[n];
        res.push(p);
        while n % p == 0 {
            n /= p;
        }
    }
    res
}


fn phi(n: usize, sieve: &[usize]) -> usize {
    let factors = factors(n, sieve);
    let mut res = n;
    for p in factors.iter() {
        res = res / p * (p - 1);
    }
    res
}


fn is_permutation(a: usize, b: usize) -> bool {
    let mut a = a;
    let mut b = b;
    let mut a_digits = [0; 10];
    let mut b_digits = [0; 10];
    while a > 0 {
        a_digits[a % 10] += 1;
        a /= 10;
    }
    while b > 0 {
        b_digits[b % 10] += 1;
        b /= 10;
    }
    a_digits == b_digits
}

fn main() {
    const LIM: usize = 10_000_000 + 1;
    let mut sieve = (0..=LIM).collect_vec();
    sieve[0] = 0;
    sieve[1] = 1;
    for i in 2..LIM {
        if sieve[i] == i {
            for j in (2 * i..LIM).step_by(i) {
                sieve[j] = i;
            }
        }
    }

    let (ans, phians) = (2..LIM - 1).map(|n| (n, phi(n, &sieve))).filter(|&(a, b)| is_permutation(a, b)).min_by(|a, b| (a.0 * b.1).cmp(&(a.1 * b.0))).unwrap();
    println!("min: {} (with value {})", ans, phians);
}