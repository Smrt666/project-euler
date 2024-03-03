fn gcd(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}

fn lcm(a: u32, b: u32) -> u32 {
    return a / gcd(a, b) * b;
}

fn range_lcm(n: u32) -> u32 {
    let mut r = 1;
    for i in 2..=n {
        r = lcm(r, i);
    }
    return r;
}

fn main() {
    println!("{}", range_lcm(10));
    println!("{}", range_lcm(20));
}