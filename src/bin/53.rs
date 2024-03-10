use std::cmp::min;

fn main() {
    let mut c = vec![vec![0; 101]; 101];
    for i in 0..=100 {
        c[i][0] = 1;
        c[0][i] = 1;
    }
    let mut count = 0;
    for i in 1..=100 {
        for j in 1..=100-i {
            c[i][j] = min(c[i - 1][j] + c[i][j - 1], 1_000_001);
            if c[i][j] > 1_000_000 {
                count += 1;
            }
        }
    }
    println!("{}", count);
}