fn main() {
    let mut sieve = [true; 1000000];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..sieve.len() {
        if sieve[i] {
            for j in (i * i..sieve.len()).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    let primes = sieve.iter().enumerate().filter(|(_, &x)| x).map(|(i, _)| i).collect::<Vec<usize>>();
    let mut ans = 0;
    let mut longest = 0;
    for i in 0..primes.len() {
        let mut sum = 0;
        for j in i..primes.len() {
            sum += primes[j];
            if sum >= 1_000_000 {
                break;
            }
            if sieve[sum] && j - i > longest {
                ans = sum;
                longest = j - i;
            }
        }
    }
    println!("{}", ans);
}