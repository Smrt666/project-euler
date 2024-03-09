fn spiral_sum(n: i32) -> i32 {
    let mut sum = 1;
    let mut a = 1;
    for d in (2..=n-1).step_by(2) {
        for _ in 0..4 {
            a += d;
            sum += a;
        }
    }
    sum
}

fn main() {
    println!("{}", spiral_sum(5));
    println!("{}", spiral_sum(1001));
}