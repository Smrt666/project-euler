fn digit_len(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 3,
        2 => 3,
        3 => 5,
        4 => 4,
        5 => 4,
        6 => 3,
        7 => 5,
        8 => 5,
        9 => 4,
        _ => panic!("Invalid digit: {}", n),        
    }
}

fn tens_len(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 3,
        2 => 6,
        3 => 6,
        4 => 5,
        5 => 5,
        6 => 5,
        7 => 7,
        8 => 6,
        9 => 6,
        _ => panic!("Invalid tens: {}", n),        
    }
}

fn to_thousand_len(n: u32) -> u32 {
    if n == 1000 {
        return 11;
    }
    let hundreds = n / 100;
    let tens = n % 100 / 10;
    let ones = n % 10;

    let mut len = digit_len(ones);
    let tl = tens_len(tens);
    len = match n % 100 {
        11 => 6,
        12 => 6,
        13 => 8,
        14 => 8,
        15 => 7,
        16 => 7,
        17 => 9,
        18 => 8,
        19 => 8,
        _ => len + tl,
    };

    if hundreds > 0 {
        if len > 0 {
            len += digit_len(hundreds) + 10;
        } else {
            len += digit_len(hundreds) + 7;
        }
    }

    return len;
}

fn main() {
    let mut sum = 0;
    for i in 1..=1000 {
        sum += to_thousand_len(i);
    }
    println!("1000 sum: {}", sum);
    println!("342: {}", to_thousand_len(342));
    println!("115: {}", to_thousand_len(115));
}