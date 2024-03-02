fn main() {
    // special pythagorean triplet
    let sum = 1000;
    for a in 1..sum {
        for b in a + 1..sum {
            let c = sum - a - b;
            if a * a + b * b == c * c {
                println!("{}", a * b * c);
                return;
            }
        }
    }
}