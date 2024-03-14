use std::collections::HashSet;

fn digits_fsum(n: u32) -> u32 {
    let mut sum: u32 = 0;
    let mut n = n;
    while n > 0 {
        sum += (1..=(n % 10)).product::<u32>();
        n /= 10;
    }
    sum
}

fn rad_len(n: u32) -> u32 {
    let mut found = HashSet::new();
    found.insert(n);
    let mut next = digits_fsum(n);
    // print!("{} -> ", n);
    while !found.contains(&next) {
        // print!("{} -> ", next);
        found.insert(next);
        next = digits_fsum(next);
    }
    // println!("({})", next);
    found.len() as u32
}


fn main() {
    let mut count = 0;
    for n in 1..1_000_000 {
        if rad_len(n) == 60 {
            count += 1;
        }
    }
    println!("{}", count);
}