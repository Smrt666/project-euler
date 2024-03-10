fn order_digits(mut n: u64) -> u64 {
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits.sort();
    digits.reverse();
    let mut result = 0;
    for d in digits {
        result = result * 10 + d;
    }
    result
}


fn main() {
    let mut i = 1;
    loop {
        let mut j = 2;
        let od = order_digits(i);
        while j <= 6 {
            if od != order_digits(i * j) {
                break;
            }
            j += 1;
        }
        if j == 7 {
            println!("{}", i);
            break;
        }
        i += 1;
    }
}