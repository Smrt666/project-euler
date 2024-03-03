fn next_collatz(n: u64) -> u64 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

fn collatz_len(n: u64) -> u64 {
    let mut len = 1;
    let mut i = n;
    while i != 1 {
        i = next_collatz(i);
        len += 1;
    }
    len
}

fn main() {
    let mut max_len = 0;
    let mut max_i = 0;
    for i in 1..1_000_000 {
        let len = collatz_len(i);
        if len > max_len {
            max_len = len;
            max_i = i;
        }
    }
    println!("{}", max_i);
}