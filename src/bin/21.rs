fn d(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..n {
        if n % i == 0 {
            sum += i;
        }
    }
    sum
}

fn main() {
    let mut sum = 0;
    for i in 1..10000 {
        let a = d(i);
        let b = d(a);
        if i == b && a != i {
            sum += i;
        }
    }
    println!("d(220)={} d(284)={}", d(220), d(284));
    println!("{}", sum);
}