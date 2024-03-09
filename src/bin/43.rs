use itertools::Itertools;

fn main() {
    let divisors = vec![2, 3, 5, 7, 11, 13, 17];
    let mut sum: u64 = 0;
    for p in (0..10).permutations(10) {
        if p[0] == 0 {
            continue;
        }
        if (0..divisors.len()).all(|i| p[i + 1 ..= i + 3].iter().fold(0, |acc, &x| acc * 10 + x) % divisors[i] == 0) {
            let n = p.iter().fold(0, |acc, &x| acc * 10 + x);
            println!("{}", n);
            sum += n;
        }
    }
    println!("sum = {}", sum);
}