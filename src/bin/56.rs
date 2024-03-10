use num_bigint::BigUint;

fn dsum(n: BigUint) -> u32 {
    n.to_str_radix(10)
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum()
}

fn main() {
    let mut max = 0;
    for a in 1..100 {
        for b in 1..100 {
            let n = BigUint::from(a as u32).pow(b);
            let sum = dsum(n);
            if sum > max {
                max = sum;
            }
        }
    }
    println!("{}", max);
}