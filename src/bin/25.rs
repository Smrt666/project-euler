use num_bigint::BigUint;

fn main() {
    let mut a = BigUint::from(1 as u32);
    let mut b = BigUint::from(1 as u32);

    let mut i = 2;
    while b.clone().to_string().len() < 1000 {
        let tmp = a;
        a = b.clone();
        b += tmp;

        i += 1;
    }

    println!("{}", i);
}