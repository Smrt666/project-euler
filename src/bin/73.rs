use gcd::Gcd;
use std::collections::HashSet;


fn reduce(a: usize, b: usize) -> (usize, usize) {
    let g = a.gcd(b);
    (a / g, b / g)
}


fn main() {
    let d = 12000;
    let mut set = HashSet::new();
    for a in 1..d {
        for b in a + 1..=d {
            let (n, m) = reduce(a, b);
            if n * 2 < m && n * 3 > m {
                set.insert((n, m));
            }
        }
    }
    println!("{}", set.len());
}