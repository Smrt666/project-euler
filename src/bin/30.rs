fn dk_sum(n: u32, k: u32) -> u32 {
    let mut n = n;
    let mut sum = 0;
    while n > 0 {
        sum += (n % 10).pow(k);
        n /= 10;
    }
    sum
}

fn find_sum(k: u32) -> u32 {
    let mut sum = 0;
    // Glede na račune na papirju: 
    // meja deluje vsaj za k <= 8, za k <= 95 deluje 10^(k + 2)
    for n in 2..10_u32.pow(k + 1) {
        if n == dk_sum(n, k) {
            sum += n;
        }
    }
    sum
}

fn main() {
    println!("{}", find_sum(4));
    println!("{}", find_sum(5));
}