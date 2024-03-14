use fraction::Fraction;


fn main() {
    let d = 1000_000;
    let mut l = Fraction::new(2_u64, 7_u64);
    let r = Fraction::new(3_u64, 7_u64);
    let mut den = 8;
    while den <= d {
        let candidate = Fraction::new((l.numer().unwrap() * den + l.denom().unwrap()) / l.denom().unwrap(), den);
        if candidate < r {
            l = candidate;
        }
        den += 1;
    }
    println!("{}", l);
}