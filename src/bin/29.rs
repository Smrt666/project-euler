use num_bigint::BigUint;


fn count_different(n: u32) -> usize {
    let mut v = Vec::<BigUint>::new();
    for a in 2..=n {
        for b in 2..=n {
            let n = BigUint::from(a).pow(b);
            if !v.contains(&n) {
                v.push(n);
            }
        }
    }
    return v.len();
}

fn main() {
    println!("{}", count_different(5));
    println!("{}", count_different(100));
}