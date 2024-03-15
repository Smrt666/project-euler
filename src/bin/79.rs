use std::fs;
use std::env;
use itertools::Itertools;


fn main() {
    println!("Path = {}", env::current_dir().unwrap().display());
    let contents = fs::read_to_string("inputs/79_keylog.txt").expect("File not found");
    let rules = contents.lines().map(|line| line.chars().map(|x| x.to_digit(10).unwrap() as usize).collect::<Vec<usize>>()).collect::<Vec<Vec<usize>>>();

    let mut digits = vec![false; 10];
    for rule in rules.iter() {
        for &d in rule.iter() {
            digits[d] = true;
        }
    }

    let digits = digits.iter().enumerate().filter(|(_, &x)| x).map(|(i, _)| i).collect::<Vec<usize>>();

    let mut stop = false;
    for l in 5.. {
        for passwort in (0..l).map(|_| digits.iter().map(|&x| x)).multi_cartesian_product() {
            let mut valid = true;
            for rule in rules.iter() {
                let mut i = 0;
                for &d in passwort.iter() {
                    if d == rule[i] {
                        i += 1;
                    }
                    if i == 3 {
                        break;
                    }
                }
                if i != 3 {
                    valid = false;
                    break;
                }
            }
            if valid {
                println!("{}", passwort.iter().map(|d| d.to_string()).collect::<String>());
                stop = true;
                break;
            }
        }
        if stop {
            break;
        }
    }

}