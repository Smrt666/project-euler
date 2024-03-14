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


fn main() {
    const LIM: usize = 1000_000 + 1;
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

    let ans = (2..LIM).map(|n| phi(n, &sieve)).sum::<usize>();
    println!("{}", ans);
}