fn digcount(n: u128) -> u32 {
    let mut count = 0;
    let mut n = n;
    while n > 0 {
        count += 1;
        n /= 10;
    }
    count
}


fn main() {
    let mut counter = 0;
    for a in 1..10 as u128 {
        let mut n = 1;
        let mut lhs = 1;
        let mut rhs = a;
        while lhs <= rhs {
            if digcount(a.pow(n)) == n {
                counter += 1;
            }
            lhs *= 10;
            rhs *= a;
            n += 1;
        }
    }
    println!("count = {}", counter);
}