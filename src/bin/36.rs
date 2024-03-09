fn is_palindorme_b(n: u32, b: u32) -> bool {
    let mut nc = n;
    let mut rev = 0;
    while nc > 0 {
        rev = rev * b + nc % b;
        nc /= b;
    }
    rev == n
}

fn main() {
    println!("{}", (1..1000000).filter(|&x| is_palindorme_b(x, 10) && is_palindorme_b(x, 2)).sum::<u32>());
}