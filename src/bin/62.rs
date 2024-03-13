use std::collections::HashMap;

fn orddigs(mut n: u64) -> Vec<u8> {
    let mut digs = Vec::new();
    while n > 0 {
        digs.push((n % 10) as u8);
        n /= 10;
    }
    digs.sort();
    digs
}

fn main() {
    let mut counts: HashMap<Vec<u8>, (u64, u32)> = HashMap::new();
    for n in 1..10_u64.pow(5) {
        let cb = n * n * n;
        let digs = orddigs(cb);
        let entry = counts.entry(digs).or_insert((cb, 0));
        entry.1 += 1;
    }

    if let Some(res) = counts.values().filter(|(_, c)| *c == 5).min() {
        println!("{}", res.0);
    } else {
        println!("No solution found");
    }
}