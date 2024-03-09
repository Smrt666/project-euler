fn get_digits(n: u32) -> Vec<u16> {
    let mut digits: Vec<u16> = Vec::new();
    let mut n = n;
    while n > 0 {
        digits.push((n % 10) as u16);
        n /= 10;
    }
    digits.reverse();
    digits
}

fn main() {
    let mut digits: Vec<u16> = vec![0];
    for i in 0..1000000 as u32 {
        digits.extend(get_digits(i).iter());
    }
    println!("{}", (0..=6).map(|x| digits[(10 as usize).pow(x)] as u32).product::<u32>());
}