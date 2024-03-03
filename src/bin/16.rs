use std::ops;
use std::fmt;


#[derive(Clone)]
struct BigNumber {
    digits: Vec<u8>,
}

impl BigNumber {
    fn new(n: u64) -> Self {
        let mut digits = vec![];
        let mut n = n;
        while n > 0 {
            digits.push((n % 10) as u8);
            n /= 10;
        }
        return BigNumber { digits: digits };
    }

    fn pow(&self, n: u32) -> Self {
        let mut n = n;
        let mut result = BigNumber { digits: vec![1] };
        let mut multiplier = self.clone();
        while n > 0 {
            if n % 2 == 1 {
                result = &result * &multiplier;
            }
            multiplier = &multiplier * &multiplier;
            n /= 2;
        }
        return result;
    }
}


impl ops::Add for &BigNumber {
    type Output = BigNumber;

    fn add(self, other: Self) -> BigNumber {
        let mut carry = 0;
        let mut result = vec![];
        for digit in 0..self.digits.len().max(other.digits.len()) {
            let sum = self.digits.get(digit).unwrap_or(&0) + other.digits.get(digit).unwrap_or(&0) + carry;
            result.push(sum % 10);
            carry = sum / 10;
        }
        while carry > 0 {
            result.push(carry % 10);
            carry /= 10;
        }

        return BigNumber { digits: result };
        
    }
}

impl ops::Mul for &BigNumber {
    type Output = BigNumber;

    fn mul(self, other: Self) -> BigNumber {
        let mut result = BigNumber { digits: vec![0] };
        for (i, digit) in self.digits.iter().enumerate() {
            let mut carry = 0;
            let mut partial = vec![0; i];
            for other_digit in other.digits.iter() {
                let product = digit * other_digit + carry;
                partial.push(product % 10);
                carry = product / 10;
            }
            while carry > 0 {
                partial.push(carry % 10);
                carry /= 10;
            }
            result = &result + &BigNumber { digits: partial };
        }
        return result;
    }
}


impl fmt::Display for BigNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for digit in self.digits.iter().rev() {
            result.push_str(&digit.to_string());
        }
        write!(f, "{}", result)
    }
}


fn main() {
    let a = BigNumber::new(123);
    let b = BigNumber::new(456);
    println!("{}", &a + &b);
    println!("{}", &a * &b);
    println!("{}", BigNumber::new(2).pow(100));
    println!("{}", BigNumber::new(2).pow(1000));

    let mut dsum: u32 = 0;
    for digit in BigNumber::new(2).pow(1000).digits.iter() {
        dsum += *digit as u32;
    }

    println!("digit sum of 2**1000 = {}", dsum);
}