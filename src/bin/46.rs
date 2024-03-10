fn main() {
    const LIM: usize = 100000;
    let mut sieve = [true; LIM];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..LIM {
        if sieve[i] {
            for j in (i * i..LIM).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    for i in (3..LIM).step_by(2) {
        if sieve[i] {
            continue;
        }
        let mut found = false;
        for j in 1..=((i / 2) as f64).sqrt() as usize {
            if sieve[i - 2 * j * j] {
                found = true;
                break;
            }
        }
        if !found {
            println!("{}", i);
            break;
        }
    }
    println!("Not found");
}