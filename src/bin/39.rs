
fn nos(p: u32) -> u32 {
    let mut count = 0;
    for i in 1..=(((2 * p) as f64).sqrt() as u32) {
        if 2 * p % i == 0 {
            let a = i;
            let b = 2 * p / i;
            let c = ((a * a + b * b) as f64).sqrt() as u32;
            if a * a + b * b == c * c {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    println!("{}", (1..=1000).max_by_key(|&x| nos(x)).unwrap());
}