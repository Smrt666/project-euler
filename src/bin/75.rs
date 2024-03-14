use std::collections::HashMap;

use itertools::Itertools;


fn factor_square(n: u64, sieve: &Vec<u64>) -> Vec<u64> {
    // vrne vse delitelje števila n^2, ki so manjši od n
    let mut count: HashMap<u64, u32> = HashMap::new();
    let mut m = n;
    while m > 1 {
        let p = sieve[m as usize];
        *count.entry(p).or_insert(0) += 1;
        m /= p;
    }
    let mut res = vec![];
    for divisor in count.keys().into_iter().map(|d| (0..=count[d] * 2).map(|p| d.pow(p))).multi_cartesian_product() {
        let actual_divisor = divisor.iter().product();
        if actual_divisor < n {
            res.push(actual_divisor);
        }
    }
    res
}


fn main() {
    const L: u64 = 1500000;
    let mut count: HashMap<u64, u32> = HashMap::new();
    
    let mut sieve = (0..=L).collect_vec();
    for m in 2..=((L as f64).sqrt() as u64) {
        if sieve[m as usize] == m {
            for k in (m * m..=L).step_by(m as usize) {
                sieve[k as usize] = m;
            }
        }
    }

    // for n in 1..15 {
    //     println!("{} : {:?}", n * n, factor_square(n, &sieve));
    // }

    for a in 2..L / 2 {
        for p in factor_square(a, &sieve) {
            let q = a * a / p;
            let b = (q - p) / 2;
            let c = (q + p) / 2;
            if a < b && a + b + c <= L && a * a + b * b == c * c {
                *count.entry(a + b + c).or_insert(0) += 1;
            }
        }
        if a % 100000 == 0 {
            println!("{} %", (a as f64 / L as f64 * 200.0) as u32);
        }
    }
    println!("{}", count.values().filter(|&&v| v == 1).count());
    println!("{}", count[&12]);
}