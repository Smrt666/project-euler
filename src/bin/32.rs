use std::{collections::HashSet, vec};

fn is_panp(a: u64, b: u64) -> bool {
    let p = a * b;
    let mut used = vec![false; 10];
    for n in [a, b, p] {
        let mut n = n;
        while n > 0 {
            let d = n % 10;
            if d == 0 || used[d as usize] {
                return false;
            }
            used[d as usize] = true;
            n /= 10;
        }
    }

    return used.iter().enumerate().all(|(i, &x)| if i == 0 { true } else { x });
}

fn main() {
    let mut products = HashSet::new();
    for a in 1..10_u64.pow(5) {
        for b in a..10_u64.pow(5) {
            if a * b > 10_u64.pow(5) {
                break;
            }
            if is_panp(a, b) {
                products.insert(a * b);
            }
        }
    }

    println!("{}", products.iter().sum::<u64>());
}