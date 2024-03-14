fn pentagonal(n: i128) -> i128 {
    n * (3 * n - 1) / 2
}

fn main() {
    let mut p: Vec<i32> = vec![1];
    for n in 1.. {
        p.push(0);
        for k in 1..n + 1 {
            let sign = if k % 2 == 0 { -1 } else { 1 };
            let mut stop = true;
            for t in [pentagonal(k), pentagonal(-k)] {
                if n >= t {
                    p[n as usize] += sign * p[(n - t) as usize];
                    stop = false;
                }
            }
            if stop {
                break;
            }
        }
        p[n as usize] %= 1_000_000;
        if p[n as usize] == 0 {
            println!("{}", n);
            break;
        }
    }
}