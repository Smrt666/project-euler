fn modpow(b: u128, e: u128, m: u128) -> u128 {
    let mut result = 1;
    let mut base = b % m;
    let mut exp = e;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % m;
        }
        exp /= 2;
        base = (base * base) % m;
    }
    result

}

fn main() {
    let mut sum = 0;
    let m: u128 = 10_000_000_000;
    for i in 1..=1000 {
        sum += modpow(i, i, m);
        sum %= m;
    }
    println!("{}", sum);
}