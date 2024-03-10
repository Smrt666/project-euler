fn is_palindrome(n: u128) -> bool {
    let mut rev = 0;
    let mut m = n;
    while m > 0 {
        rev = rev * 10 + m % 10;
        m /= 10;
    }
    n == rev
}


fn is_lychrel(n: u128) -> bool {
    let mut n = n;
    for _ in 0..50 {
        let mut rev = 0;
        let mut m = n;
        while m > 0 {
            rev = rev * 10 + m % 10;
            m /= 10;
        }
        n += rev;
        if is_palindrome(n) {
            return false;
        }
    }
    true
}

fn main() {
    let mut count = 0;
    for i in 1..10_000 {
        if is_lychrel(i) {
            count += 1;
        }
    }
    println!("{}", count);
}