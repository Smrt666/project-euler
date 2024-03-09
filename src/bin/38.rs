use std::vec;

fn digits(n: u32) -> Vec<u32> {
    let mut v = Vec::new();
    let mut x = n;
    while x > 0 {
        v.push(x % 10);
        x /= 10;
    }
    v
}

fn is_pancon(n: u32) -> (bool, u32) {
    let mut used = vec![false; 10];
    for i in 1..=9 {
        for d in digits(i*n) {
            if used[d as usize] {
                return (false, 0);
            }
            used[d as usize] = true;
        }
        if used[0] {
            return (false, 0);
        }
        if used[1..].iter().all(|&x| x) {
            return (true, i);
        }
    }
    (false, 0)
}

fn main() {
    let mut max_pan: Vec<u32> = vec![0];
    for n in 1..1000000 {
        let (is_pan, i) = is_pancon(n);
        if is_pan && i > 1 {
            let tmp = (1..=i).map(|x| {let mut digs = digits(x*n); digs.reverse(); digs}).collect::<Vec<Vec<u32>>>();
            let tmp = tmp.iter().flatten().map(|&x| x).collect();
            println!("{}: {:?}", n, tmp);
            if tmp > max_pan {
                max_pan = tmp;
            }
        }
    }
    let max_pan = max_pan.iter().map(|x| x.to_string()).collect::<String>();
    println!("{}", max_pan); // not 962783541 not 987654321 not 981726354
}