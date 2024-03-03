fn number_of_divisors(n: u64) -> u32 {
    let mut count = 0;
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            count += 2;
        }
        i += 1;
    }
    return count;
}


fn first_over(c: u32) -> u64 {
    let mut n = 0;
    let mut i = 1;
    loop {
        n += i;
        i += 1;
        if number_of_divisors(n) > c {
            return n;
        }
    }
}

fn main() {
    println!("{}", first_over(5));
    println!("{}", first_over(500));
}