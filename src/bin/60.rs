use std::collections::{HashMap, HashSet};

fn concat(a: usize, b: usize) -> usize {
    let mut c = b;
    let mut a = a;
    while c > 0 {
        a *= 10;
        c /= 10;
    }
    a + b
}

fn intconcat2_isprime(a: usize, b: usize, sieve: &Vec<bool>) -> bool {
    let n = concat(a, b);
    let m = concat(b, a);
    n < sieve.len() && m < sieve.len() && sieve[n] && sieve[m]
}

fn main() {
    const LIM: usize = 100_000_000;
    let mut sieve = vec![true; LIM];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..(LIM as f64).sqrt() as usize {
        if sieve[i] {
            for j in (i * i..LIM).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    let primes = (0..10_000).filter(|&x| sieve[x]).collect::<Vec<_>>();
    let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (i, &a) in primes.iter().enumerate() {
        for &b in primes[i + 1..].iter() {
            if intconcat2_isprime(a, b, &sieve) {
                graph.entry(a).or_insert(HashSet::new()).insert(b);
                graph.entry(b).or_insert(HashSet::new()).insert(a);
            }
        }
    }

    for &p in primes.iter() {
        graph.entry(p).or_insert(HashSet::new()).insert(p);
    }

    println!("Built graph");


    let mut minsum = usize::MAX;
    for &p in primes.iter() {
        if p > minsum {
            break;
        }
        let tranp = graph[&p].clone();
        for &q in tranp.iter().filter(|&&x| x > p) {
            if q + p > minsum {
                break;
            }
            let tranqp = graph[&q].intersection(&tranp).map(|&x| x).collect::<HashSet<usize>>();
            for &r in tranqp.iter().filter(|&&x| x > q) {
                if r + q + p > minsum {
                    break;
                }
                let tranrpq = graph[&r].intersection(&tranqp).map(|&x| x).collect::<HashSet<usize>>();
                for &s in tranrpq.iter().filter(|&&x| x > r) {
                    if s + r + q + p > minsum {
                        break;
                    }
                    let tranrpqs = graph[&s].intersection(&tranrpq).map(|&x| x).collect::<HashSet<usize>>();
                    // println!("{p},{q},{r},{s} ~ {:?}", tranrpqs);
                    if let Some(min) = tranrpqs.iter().filter(|&&x| x > s).min() {
                        let sum = p + q + r + s + min;
                        if sum < minsum {
                            println!("sum = {} : {} {} {} {} {}", sum,  p, q, r, s, min);
                            minsum = sum;
                        }
                    }
                }
            }
        }
    }
    println!("minsum = {}", minsum);
}