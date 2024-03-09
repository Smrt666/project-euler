fn div25(n: u32) -> u32 {
    let mut r = n;
    for d in [2, 5] {
        while r % d == 0 {
            r /= d;
        }
    }
    r
}


fn digit_cyc_len(n: u32) -> u32 {
    let n = div25(n);
    if n == 1 {
        return 1;
    }

    let mut r = 1;
    let mut p = 10 % n;
    while p % n != 1 {
        p = (p * 10) % n;
        r += 1;
    }

    r
}


fn main() {
    let mut max = 0;
    let mut max_i = 0;
    for i in 1..1000 {
        let l = digit_cyc_len(i);
        if l > max {
            max = l;
            max_i = i;
        }
    }

    println!("{}", max_i);

    for i in 1..20 {
        println!("{}: {}", i, digit_cyc_len(i));
    }
}