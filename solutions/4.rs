fn is_palindromic(n: u32) -> bool {
    let mut reversed = 0;
    let mut nc = n;
    while nc > 0 {
        reversed = reversed * 10 + nc % 10;
        nc /= 10;
    }

    return n == reversed;
}

fn largest_palindromic_product(n: u32) -> u32 {
    let mut r = 0;
    for a in 10_u32.pow(n - 1)..10_u32.pow(n) {
        for b in a..10_u32.pow(n) {
            let p = a * b;
            if p > r && is_palindromic(p) {
                r = p;
            }
        }
    }

    return r;
}

fn main() {
    println!("{}", largest_palindromic_product(2));
    println!("{}", largest_palindromic_product(3));
}