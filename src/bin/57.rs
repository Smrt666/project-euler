use fraction::prelude::GenericFraction;
use num_bigint::BigUint;

type Fraction = GenericFraction<BigUint>;

fn main() {
    let mut exp_no1: Vec<Fraction> = vec![Fraction::new(1_u64, 2_u64)];
    for _ in 1..1000 {
        let last = exp_no1.last().unwrap();
        exp_no1.push(Fraction::new(1_u64, 1_u64) / (last.clone() + Fraction::new(2_u64, 1_u64)));
    }
    let mut count = 0;
    for fr in exp_no1 {
        let fr = fr + Fraction::new(1_u64, 1_u64);
        if fr.numer().unwrap().to_string().len() > fr.denom().unwrap().to_string().len() {
            count += 1;
        }
    }
    println!("{}", count);
}