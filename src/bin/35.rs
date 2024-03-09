fn is_circ_prime(n: u32, sieve: &Vec<bool>) -> bool {
    let mut nc = n;
    let mut nlen = 0;
    while nc > 0 {
        nlen += 1;
        nc /= 10;
    }
    nc = n;
    if !sieve[nc as usize] {
        return false;
    }
    nc = (nc % 10) * 10_u32.pow(nlen - 1) + nc / 10;
    while nc != n {
        if !sieve[nc as usize] {
            return false;
        }
        nc = (nc % 10) * 10_u32.pow(nlen - 1) + nc / 10;
    }
    true
}

fn cprimesu(n: u32, sieve: &Vec<bool>) -> u32 {
    let mut count = 0;
    for i in 2..n {
        if sieve[i as usize] && is_circ_prime(i as u32, &sieve) {
            count += 1;
        }
    }
    count
}

fn main() {
    let mut sieve = vec![true; 1000000];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..1000000 {
        if sieve[i] {
            for j in (i*i..1000000).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    
    println!("{}", cprimesu(100, &sieve));
    println!("{}", cprimesu(1000000, &sieve));
}