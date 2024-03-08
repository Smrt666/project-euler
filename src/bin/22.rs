use std::fs;
use std::env;

fn main() {
    println!("Path = {}", env::current_dir().unwrap().display());
    let contents = fs::read_to_string("inputs/22_names.txt").expect("File not found");

    let mut names = contents.split(",").map(|x| x.replace("\"", "")).collect::<Vec<String>>();
    names.sort();

    let sum = names.iter().enumerate().map(|(i, name)| {
        let score = name.chars().map(|c| c as i32 - 64).sum::<i32>();
        score * (i as i32 + 1)
    }).sum::<i32>();

    println!("Sum = {}", sum);
}
