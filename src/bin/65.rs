use fraction::prelude::BigFraction;

fn main() {
    let mut r = BigFraction::new(0_u32, 1_u32);
    for i in (2..=101).rev() {
        let p: u32 = if i % 3 == 0 {
            2 * i / 3
        } else {
            1
        };

        r = BigFraction::new(p, 1_u32) + r.recip();
    }
    r = BigFraction::new(2_u32, 1_u32) + r.recip();
    println!("{}", r.numer().unwrap().to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>());
}