use gcd::Gcd;


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

fn cont_frac_len(n: u128) -> u128 {
    let sqrtn = isqrt(n);
    if sqrtn * sqrtn == n {
        return 0;
    }
    let mut a = 1;
    let mut b = 0;
    let mut c = 1;
    let mut len = 0;
    let (mut a1, mut b1, mut c1) = (a, b, c);
    // print!("sqrt({n}) >= {sqrtn} : ");
    loop {
        len += 1;
        let (_d, a_, b_, c_) = next_frac(a, b, c, n, sqrtn);
        let fgcd = (a_.abs() as u128).gcd((b_.abs() as u128).gcd(c_.abs() as u128)) as i128;
        // print!("{_d} ");
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
    // print!("\n");
    len - 1
}


fn main() {
    // let (d, a, b, c) = next_frac(1, 0, 1, 5, 2);
    // println!("d = {d}, a = {a}, b = {b}, c = {c}");
    let mut odds = 0;
    for n in 2..=10000 {
        let cf = cont_frac_len(n);
        // println!("n: {n}, cf = {cf}");
        if cf % 2 == 1 {
            odds += 1;
        }
    }
    println!("odds = {odds}");
}