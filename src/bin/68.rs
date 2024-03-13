use itertools::Itertools;


fn perm_to_str(perm: Vec<u32>) -> [[u32; 3]; 5] {
    let res = [
        [perm[0], perm[1], perm[2]],
        [perm[3], perm[2], perm[4]],
        [perm[5], perm[4], perm[6]],
        [perm[7], perm[6], perm[8]],
        [perm[9], perm[8], perm[1]],
    ];
    let min = res.iter().min();
    let min_index = res.iter().position(|&x| x == *min.unwrap()).unwrap();
    let res = res.iter().cycle().skip(min_index).take(5).map(|x| *x).collect::<Vec<[u32; 3]>>();
    res.try_into().unwrap()
}

fn is_ok(s: &[[u32; 3]; 5]) -> bool {
    let sum = s[0].iter().sum::<u32>();
    (1..5).all(|i| s[i].iter().sum::<u32>() == sum)
}


fn main() {
    let ans = (1..=10).permutations(10).map(perm_to_str).filter(is_ok).max().unwrap().map(|x| x.map(|y| y.to_string()).join("")).join("");
    println!("{}", ans);
}