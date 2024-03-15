use std::fs;
use std::env;
use std::vec;


fn main() {
    println!("Path = {}", env::current_dir().unwrap().display());
    let contents = fs::read_to_string("inputs/81_matrix.txt").expect("File not found");
    let matrix = contents.lines().map(|line| line.split(',').map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>()).collect::<Vec<Vec<usize>>>();

    let mut res = vec![vec![0; 80]; 80];
    for i in 0..80 {
        res[0][i] = matrix[0][i] + if i > 0 { res[0][i - 1] } else { 0 };
        res[i][0] = matrix[i][0] + if i > 0 { res[i - 1][0] } else { 0 };
    }

    for i in 1..80 {
        for _ in 0..80 {
            for j in 0..80 {
                res[j][i] = [
                    res[j][i]
                ]
            }
        }
    }    

    println!("{}", (0..80).map(|i| res[i][79]).min().unwrap());
}