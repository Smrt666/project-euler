use gcd::Gcd;

fn is_special(a: u32, b: u32) -> bool {
    let a1 = a / 10;
    let a2 = a % 10;
    let b1 = b / 10;
    let b2 = b % 10;
    if a2 == 0 || b2 == 0 || a1 == 0 || b1 == 0 {
        return false;
    }
    if a1 == b1 && a2 * b == a * b2 {
        return true;
    }
    if a1 == b2 && a2 * b == a * b1 {
        return true;
    }
    if a2 == b1 && a1 * b == a * b2 {
        return true;
    }
    if a2 == b2 && a1 * b == a * b1 {
        return true;
    }
    false
}

fn main() {
    let mut pa = 1;
    let mut pb = 1;
    for a in 10..100 {
        for b in a + 1..100 {
            if is_special(a, b) {
                println!("{}/{} = {}/{}", a, b, a / 10, b % 10);
                pa *= a;
                pb *= b;
                let pg = pa.gcd(pb);
                pa /= pg;
                pb /= pg;
            }
        }
    }
    println!("product : {}/{}", pa, pb);
}