use num_bigint::BigUint;


fn main() {
    println!("{}", BigUint::from(2 as u32).pow(1000).to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>());
}