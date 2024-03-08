fn nth_permutation(n: u32, v: Vec<u32>) -> Vec<u32> {
    if v.len() == 1 {
        return v;
    }

    let fact: u32 = (1..v.len()).product::<usize>() as u32;
    let mut i: usize = 0;
    while i + 1 < v.len() && n > fact * v[i + 1] {
        i += 1;
    }
    // println!("n: {}, fact: {}, i: {}, v: {:?}", n, fact, i, v);
    return [
        vec![v[i]], 
        nth_permutation(
            n - v[i] * fact, 
            [
                v[..i].to_vec(), 
                v[i + 1..].iter().map(|x| *x - 1).collect::<Vec<u32>>()
            ].concat()
        ).iter().map(|x| if *x >= i as u32 {*x + 1} else {*x}).collect()
    ].concat();
}

fn main() {
    let v: Vec<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let n: u32 = 1_000_000;
    for i in 1..10 { // debug
        println!("{:?}", nth_permutation(i, vec![0, 1, 2, 3]));
    }
    println!("{}", nth_permutation(n, v).iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
}
