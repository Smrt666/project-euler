use std::fs;
use std::env;


fn card_value(card: [char; 2]) -> u32 {
    match card[0] {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0
    }
}


fn hand_value(hand: &Vec<[u8; 2]>) -> Vec<u32> {
    let single_color = hand.iter().all(|&s| s[0] == hand[0][0]);
    let mut values = hand.iter().map(|&s| card_value([s[0] as char, s[1] as char])).collect::<Vec<u32>>();
    values.sort();
    let mut rval = values.clone();
    rval.reverse();
    if values == [10, 11, 12, 13, 14] {  // Royal flush
        if single_color {
            return [[10, 14].to_vec(), rval].concat();
        }
    }
    if values.iter().enumerate().all(|(i, &v)| v == values[0] + i as u32) {  // Straight
        if single_color {  // Straight flush
            return [[9, *values.last().unwrap()].to_vec(), rval].concat();
        } else {  // Straight
            return [[5, *values.last().unwrap()].to_vec(), rval].concat();
        }
    }
    let mut count = [0; 15];
    for &v in &values {
        count[v as usize] += 1;
    }
    let pairs = count.iter().filter(|&&c| c == 2).count();
    let threes = count.iter().filter(|&&c| c == 3).count();
    let fours = count.iter().filter(|&&c| c == 4).count();
    if fours == 1 {  // Four of a kind
        return [[8, values[2]].to_vec(), rval].concat();
    }
    if threes == 1 && pairs == 1 {  // Full house
        return [[7, *values.last().unwrap()].to_vec(), rval].concat();
    }
    if single_color {  // Flush
        return [[6, *values.last().unwrap()].to_vec(), rval].concat();
    }
    // straight je že [5] zgoraj
    if threes == 1 {  // Three of a kind
        return [[4, values[2]].to_vec(), rval].concat();
    }
    if pairs == 2 {  // Two pairs
        return [[3, values[3]].to_vec(), rval].concat();
    }
    if pairs == 1 {  // One pair
        let mv = count.iter().enumerate().filter(|&(_, &c)| c == 2).map(|(i, _)| i as u32).max().unwrap();
        return [[2, mv].to_vec(), rval].concat();
    }
    return [[1, *values.last().unwrap()].to_vec(), rval].concat();  // High card
}


fn winner(hand1: &Vec<&str>, hand2: &Vec<&str>) -> u32 {
    let hand1 = hand1.iter().map(|c| [c.chars().nth(0).unwrap() as u8, c.chars().nth(1).unwrap() as u8]).collect::<Vec<[u8; 2]>>();
    let hand2 = hand2.iter().map(|c| [c.chars().nth(0).unwrap() as u8, c.chars().nth(1).unwrap() as u8]).collect::<Vec<[u8; 2]>>();
    let hand1 = hand_value(&hand1);
    let hand2 = hand_value(&hand2);
    if hand1 > hand2 {
        // println!("{:?} > {:?}", hand1, hand2);
        return 1;
    } else {
        // println!("{:?} < {:?}", hand1, hand2);
        return 0;
    }
}

fn main() {
    println!("{}", env::current_dir().unwrap().display());
    let contents = fs::read_to_string("inputs/54_poker.txt").expect("File not found");

    let games = contents.split("\n").filter(|l| l.len() > 0).collect::<Vec<&str>>();
    let games = games.iter().map(|g| g.split(" ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    let games = games.iter().map(|g| (g[0..5].to_vec(), g[5..10].to_vec())).collect::<Vec<(Vec<&str>, Vec<&str>)>>();


    println!("{}", games.iter().map(|(h1, h2)| winner(h1, h2)).sum::<u32>());
}