fn df_sum(n: u32) -> u32 {
    let mut sum = 0;
    let mut n = n;
    while n > 0 {
        sum += (1..=n % 10).product::<u32>();
        n /= 10;
    }
    sum
}

fn main() {
    let mut sum = 0;
    for i in 3..10_u32.pow(6) {
        if i == df_sum(i) {
            sum += i;
        }
    }
    println!("{}", df_sum(145));
    println!("{}", sum);
}