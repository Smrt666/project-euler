fn main() {
    let mut r: i32 = 0;
    let mut a = 1;
    let mut b = 1;

    while a <= 4000000 {
        if a % 2 == 0 {
            r += a;
        }
        let t = a;
        a = b + a;
        b = t;
    }
    println!("{}", r);
}