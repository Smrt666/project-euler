use std::collections::HashMap;

fn integer_partitions(n: i32, k: i32, primes: &Vec<i32>, memo: &mut HashMap<(i32, i32), u32>) -> u32 {
    if memo.contains_key(&(n, k)) {
        return memo[&(n, k)];
    }

    if n == 0 {
        return 1;
    }
    if n < 0 {
        return 0;
    }

    let res = (0..=k).map(|i| integer_partitions(n - primes[i as usize], i, primes, memo)).sum();
    memo.insert((n, k), res);
    res
}

fn main() {
    let mut memo = HashMap::new();
    let mut sieve = vec![true; 1000_000];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..1000_000 {
        if sieve[i] {
            for j in (i * i..1000_000).step_by(i as usize) {
                sieve[j] = false;
            }
        }
    }
    let primes: Vec<i32> = sieve.iter().enumerate().filter(|(_, &is_prime)| is_prime).map(|(i, _)| i as i32).collect();

    let mut i = 1;
    loop {
        let res = integer_partitions(i, i, &primes, &mut memo);
        if i < 20 {
            println!("{}: {}", i, res);
        }
        if res > 5000 {
            println!("{}", i);
            break;
        }
        i += 1;
    }
}