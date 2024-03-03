use std::fs;
use std::env;

fn main() {
    println!("{}", env::current_dir().unwrap().display());
    let contents = fs::read_to_string("inputs/13.txt").expect("File not found");
    let digits = contents.lines().map(|line| line.trim().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
    println!("{:?}", digits);
    let mut result: Vec<u32> = vec![];
    for i in (0..digits[0].len()).rev() {
        let mut sum = 0;
        for j in 0..digits.len() {
            sum += digits[j][i];
        }
        result.push(sum);
        println!("pushed {}", sum);
    }

    let mut di = 0;
    while result[di] > 9 {
        if di + 2 == result.len() {
            result.push(0);
        }
        result[di + 1] += result[di] / 10;
        result[di] %= 10;
        di += 1;
    }

    for i in 2..=11 {
        print!("{}", result[result.len() - i]);
    }
    println!();
}