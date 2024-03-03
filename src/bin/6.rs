fn sum_sq_diff(n: u32) -> u32 {
    let sum = n * (n + 1) / 2;
    return sum * sum - (1..=n).map(|x| x * x).sum::<u32>();
}

fn main() {
    println!("{}", sum_sq_diff(10));
    println!("{}", sum_sq_diff(100));
}