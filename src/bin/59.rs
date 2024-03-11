use std::fs;
use std::env;

use itertools::Itertools;

fn is_english(text: Vec<u8>) -> u32 {
    let text = String::from_utf8(text).expect("Invalid UTF-8");
    let mut count = 0;
    for word in ["the", "and", "is", "are"] {
        count += text.matches(word).count() as u32;
    }
    count
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    println!("Path = {}", env::current_dir().unwrap().display());
    let contents = fs::read_to_string("inputs/59_cipher.txt").expect("File not found");

    let code = contents.split(",").map(|x| x.parse::<u8>().unwrap()).collect::<Vec<u8>>();
    for key in (0..3).map(|_| 'a' as u8 .. 'z' as u8).multi_cartesian_product() {
        let mut text = Vec::new();
        for (i, c) in code.iter().enumerate() {
            text.push(c ^ key[i % 3]);
        }
        let score = is_english(text.clone());
        if score > 25 {
            println!("Key = {:?},  score = {}", key, score);
            println!("Text = {:?}", match String::from_utf8(text.clone()) {
                Ok(s) => s,
                Err(_) => "Invalid".to_string()
            });
            println!("Sum = {}", text.iter().map(|&x| x as u32).sum::<u32>());
            // break;
        }
    }
}