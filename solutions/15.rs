use std::collections::HashMap;

fn comb(a: u64, b: u64, memo: &mut HashMap::<(u64, u64), u64>) -> u64 {
    if b == 0 || a == b {
        1
    } else {
        if let Some(&res) = memo.get(&(a, b)) {
            return res;
        } else {
            println!("{} {}", comb(a - 1, b - 1, memo), comb(a - 1, b, memo));
            let val = comb(a - 1, b - 1, memo) + comb(a - 1, b, memo);
            memo.insert((a, b), val);
            return memo[&(a, b)];
        }
    }
}

fn main() {
    let memo = &mut HashMap::<(u64, u64), u64>::new();

    println!("{}", comb(2 + 2, 2, memo));
    println!("{}", comb(20 + 20, 20, memo));
}