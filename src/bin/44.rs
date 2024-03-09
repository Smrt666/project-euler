use std::collections::HashSet;

fn main() {
    let p: HashSet<u64> = (1..10000).map(|n: u64| n * (3 * n - 1) / 2).collect::<HashSet<u64>>();
    for &pc in p.iter() {
        for &pb in p.iter() {
            if p.contains(&(pb + pc)) && p.contains(&(2 * pb + pc)) {
                println!("{}", pc);
                return;
            }
        }
    }
    println!("Not found");
}