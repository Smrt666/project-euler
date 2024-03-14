use std::collections::HashMap;

fn integer_partitions(n: i32, k: i32, memo: &mut HashMap<(i32, i32), u32>) -> u32 {
    if memo.contains_key(&(n, k)) {
        return memo[&(n, k)];
    }

    if n == 0 {
        return 1;
    }
    if n < 0 {
        return 0;
    }

    let res = (1..=k).map(|i| integer_partitions(n - i, i, memo)).sum();
    memo.insert((n, k), res);
    res
}

fn main() {
    let mut memo = HashMap::new();
    println!("{}", integer_partitions(100, 99, &mut memo));
}