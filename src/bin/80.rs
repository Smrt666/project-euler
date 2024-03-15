use fraction::GenericFraction;
use fraction::BigUint;
use fraction::ToPrimitive;

type F = GenericFraction<BigUint>;

fn main() {
    let mut ans = 0;
    for n in 2..100 {
        if [1, 4, 9, 16, 25, 36, 49, 64, 81].contains(&n) {
            continue;
        }
        let x: F = n.into();
        let sqrtx = x.sqrt(200);
        let y = (sqrtx.fract() * (0..99).fold(F::from(1), |acc, _| acc * 10)).trunc();
        let w = sqrtx.trunc().to_u32().unwrap();
        let sum = y.to_string().chars().take(99).map(|c| c.to_digit(10).unwrap()).sum::<u32>() + w;
        // println!("{}: {w}.{} => {sum}", n, y);
        ans += sum;
    }
    println!("{}", ans);
}