fn sum(n: i32) -> i32 {
    let mut r: i32 = 0;
    for i in 1..n {
        if i % 3 == 0 || i % 5 == 0 {
            r += i;
        }
    }
    return r;
}

fn main() {
    println!("n = 10 -> {}", sum(10));
    println!("n = 1000 -> {}", sum(1000));
}