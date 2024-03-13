use gcd::Gcd;
use fraction::BigFraction;
use num::BigUint;
use std::env;


fn isqrt(n: u128) -> u128 {
    if n == 1 {
        return 1;
    }
    let mut l: u128 = 0;
    let mut r: u128 = n;
    while l + 1 < r {
        let mid = (l + r) / 2;
        if (mid * mid) < n {
            l = mid;
        } else if mid * mid > n {
            r = mid;
        } else {
            return mid;
        }
    }
    l
}


fn frac_sep(a: i128, sqrtn: u128, b: i128, c: i128) -> (i128, i128, i128, i128) {
    /* (a sqrtn + b) / c -> [floor((a sqrtn + b) / c)] + ([(a sqrtn + b) % c - a sqrtn] + [a] sqrt(n)) / [c] */
    let sqrtn = sqrtn as i128;
    (
        (a * sqrtn + b) / c,
        a,
        (a * sqrtn + b) % c - a * sqrtn,
        c
    )
}

fn frac_inv(a: i128, n: u128, b: i128, c: i128) -> (i128, i128, i128) {
    /* (a + b sqrt(n)) / c -> ([c a] - [c b] sqrt(n)) / ([a^2 n - b^2])  */
    (
        a * c,
        -b * c,
        a * a * n as i128 - b * b
    )
}

fn next_frac(a: i128, b: i128, c: i128, n: u128, sqrtn: u128) -> (i128, i128, i128, i128) {
    let (d, a, b, c) = frac_sep(a, sqrtn, b, c);
    let abcgcd = (a.abs() as u128).gcd((b.abs() as u128).gcd(c.abs() as u128)) as i128;
    let (a, b, c) = (a / abcgcd, b / abcgcd, c / abcgcd);
    let (a, b, c) = frac_inv(a, n, b, c);
    (d, a, b, c)
}

fn cont_frac(n: u128) -> Vec<u128> {
    let sqrtn = isqrt(n);
    if sqrtn * sqrtn == n {
        panic!("n is a perfect square");
    }
    let mut a = 1;
    let mut b = 0;
    let mut c = 1;
    let (mut a1, mut b1, mut c1) = (a, b, c);
    let mut ret = vec![];
    let mut len = 0;
    loop {
        len += 1;
        let (d, a_, b_, c_) = next_frac(a, b, c, n, sqrtn);
        let fgcd = (a_.abs() as u128).gcd((b_.abs() as u128).gcd(c_.abs() as u128)) as i128;
        ret.push(d as u128);
        a = a_ / fgcd;
        b = b_ / fgcd;
        c = c_ / fgcd;
        if len == 1 {
            a1 = a;
            b1 = b;
            c1 = c;
        } else if a == a1 && b == b1 && c == c1 {
            break;
        }
    }
    ret
}


fn nth_convergent(n: usize, cf: &Vec<u128>) -> BigFraction {
    if n == 0 {
        return BigFraction::new(cf[0], 1_u32);
    }
    let mut res = BigFraction::new(cf[(n - 1) % (cf.len() - 1) + 1], 1_u32);
    for i in (1..n).rev() {
        res = BigFraction::new(cf[(i - 1) % (cf.len() - 1) + 1], 1_u32) + res.recip();
    }

    BigFraction::new(cf[0], 1_u32) + res.recip()
}


fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let mut maxx = BigUint::from(0_u32);
    let mut dans = 0;
    for n in 2..=1000 {
        if isqrt(n) * isqrt(n) == n {
            continue;
        }
        let cf = cont_frac(n);
        let i = if (cf.len() - 1) % 2 == 0 {
            cf.len() - 2
        } else {
            cf.len() * 2 - 3
        };
        let frac = nth_convergent(i, &cf);
        println!("D = {}:  x = {}, cf = {:?}", n, frac.numer().unwrap(), cf);
        let (x, y) = (frac.numer().unwrap(), frac.denom().unwrap());
        if x * x != n as u32 * y * y + BigUint::from(1_u32) {
            println!("Problem with D = {}", n);
            return;
        }
        if x.gt(&maxx) {
            maxx = x.clone();
            dans = n;
        }
    }
    println!("{}", dans);
}