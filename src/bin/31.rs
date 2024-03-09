use std::collections::HashMap;

fn partitions(n: i32, v: Vec<u32>, memo: &mut HashMap<(i32, Vec<u32>), u32>) -> u32 {
    if n < 0 {
        return 0;
    } else if n == 0 || n == 1 {
        return 1;
    }

    if memo.contains_key(&(n, v.clone())) {
        return memo[&(n, v)];
    }

    v.iter().enumerate().map(|(i, &x)| {
        partitions(n - x as i32, v[0..=i].to_vec(), memo)
    }).sum()
}

fn main() {
    let mut memo = HashMap::new();
    let v = vec![1, 2, 5, 10, 20, 50, 100, 200];
    println!("{}", partitions(200, v, &mut memo));
}