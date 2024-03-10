fn getdigits(n: u32) -> Vec<u32> {
    let mut n = n;
    let mut result = vec![];
    while n > 0 {
        result.push(n % 10);
        n /= 10;
    }
    result.reverse();
    result
}

fn fromdigits(digits: &Vec<u32>) -> usize {
    let mut result = 0;
    for d in digits {
        result = result * 10 + d;
    }
    result as usize
}


fn replace_digits(digits: &Vec<u32>, indices: Vec<u32>, replacement: u32) -> Vec<u32> {
    let mut result = digits.clone();
    for i in indices {
        result[i as usize] = replacement;
    }
    result
}


fn get_replacement_positions(n: usize, count: usize) -> Vec<Vec<u32>> {
    if count == n {
        return vec![(0..n as u32).collect()];
    }
    if count == 0 {
        return vec![vec![]];
    }
    if n < count {
        return vec![];
    }

    let mut result = vec![];
    for i in 0..=(n - count) as u32 {
        let mut subresult = get_replacement_positions(n - i as usize - 1, count - 1);
        subresult = subresult.iter().map(|v| v.iter().map(|&x| x + i + 1).collect()).collect();
        for sr in &mut subresult {
            sr.push(i);
        }
        result.append(&mut subresult);
    }
    result
}


fn main() {
    const LIM: usize = 1_000_000;
    let mut sieve = vec![true; LIM];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..LIM {
        if sieve[i] {
            for j in (i * i..LIM).step_by(i) {
                sieve[j] = false;
            }
        }
    }
    let primes: Vec<usize> = sieve.iter().enumerate().filter_map(|(i, &p)| if p { Some(i) } else { None }).collect();
    let mut result = 0;
    for prime in primes {
        for count in 1..getdigits(prime as u32).len() {
            let positions = get_replacement_positions(getdigits(prime as u32).len(), count);
            for replacement in positions {
                let mut smallest = LIM;
                let mut oks = 0;
                for r in 0..10 {
                    let replaced = replace_digits(&getdigits(prime as u32), replacement.clone(), r);
                    if replaced[0] == 0 {
                        continue;
                    }
                    let replaced = fromdigits(&replaced);
                    if sieve[replaced] {
                        // if prime == 120383 {
                        //     println!("r: {:?}", replaced);
                        // }
                        if replaced < smallest {
                            smallest = replaced;
                        }
                        oks += 1;
                    }
                }
                // if prime == 120383 {
                //     println!("smallest: {}, oks: {}", smallest, oks);
                // }
                if oks == 8 {
                    result = smallest;
                    break;
                }
                // if oks > 8 {
                //     println!("prime: {}, count: {}, oks: {}", smallest, count, oks);
                // }
            }
            if result != 0 {
                break;
            }
        }
        if result != 0 {
            break;
        }
    }
    println!("result = {}", result);  // not 222823, not 120383
    // println!("rpos: {:?}", get_replacement_positions(5, 3));
}