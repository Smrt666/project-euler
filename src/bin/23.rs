fn divsum(n: i32) -> i32 {
    (1..n).map(|x| if n % x == 0 {x} else {0}).sum()
}

fn main() {
    let abundant = (1..28124).filter(|x| divsum(*x) > *x).collect::<Vec<i32>>();
    let mut ok = vec![false; 28124];
    for a in abundant.iter() {
        for b in abundant.iter() {
            if *a < *b {
                continue;
            }
            if *a + *b < 28124 {
                ok[(a + b) as usize] = true;
            }
        }
    }
    let ans = ok.iter().enumerate().map(|(i, x)| if !x {i as i32} else {0}).sum::<i32>();
    println!("{}", ans);  // 
}