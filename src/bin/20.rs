use num_bigint::BigUint;

fn main() {
    println!("{}", (1..=100).map(|n| BigUint::from(n as u32)).product::<BigUint>().to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>());
}