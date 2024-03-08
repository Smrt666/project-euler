use std::fs;
use std::env;

fn main() {
    println!("Path = {}", env::current_dir().unwrap().display());
    let contents = fs::read_to_string("inputs/67_triangle.txt").expect("File not found");
    let table = contents.lines().map(|line| line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    let mut mx = table.iter().map(|x| x.iter().map(|_| 0).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();

    for y in (0..mx.len()).rev() {
        for x in 0..mx[y].len() {
            if y == mx.len() - 1 {
                mx[y][x] = table[y][x];
            } else {
                mx[y][x] = table[y][x] + mx[y + 1][x].max(mx[y + 1][x + 1]);
            }
        }
    }

    println!("Max sum: {}", mx[0][0]);
}