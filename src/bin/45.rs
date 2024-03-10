fn main() {
    let mut pi: u64 = 165;
    let mut hi: u64 = 143;
    let mut p: u64 = 40755;
    let mut h: u64 = 40755;

    let mut i: u64 = 286;
    let mut t: u64 = i * (i + 1) / 2;

    while t != p || t != h {
        while p < t {
            pi += 1;
            p = pi * (3 * pi - 1) / 2;
        }
        while h < t {
            hi += 1;
            h = hi * (2 * hi - 1);
        }
        while t < p || t < h {
            i += 1;
            t = i * (i + 1) / 2;
        }
    }

    println!("{}", t);
}