use std::collections::HashSet;
use std::fs;
use std::env;

fn main() {
    println!("{}", env::current_dir().unwrap().display());
    let contents = fs::read_to_string("inputs/42_words.txt").expect("File not found");
    let words: Vec<String> = contents.split(',').map(|w| w[1..w.len()-1].to_string()).collect::<Vec<String>>();
    let trinums: HashSet<u32> = (1..100).map(|n| n * (n + 1) / 2).collect::<HashSet<u32>>();

    let mut count = 0;
    for word in words {
        let score = word.chars().map(|c| c as u32 - 'A' as u32 + 1).sum::<u32>();
        if trinums.contains(&score) {
            count += 1;
        }
    }
    println!("{}", count);
}