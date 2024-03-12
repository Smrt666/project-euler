use std::collections::{HashMap, HashSet};

use itertools::Itertools;


fn is_connected(a: u32, b: u32) -> bool {
    a % 100 == b / 100 && a != b
}


fn dfs_sum(start: u32, target: u32, graph: &HashMap<u32, HashSet<(usize, u32)>>, used: Vec<u32>, layers_to_use: Vec<usize>) -> i32 {
    if !graph.contains_key(&start) {
        return -1;
    }
    if layers_to_use.len() == 0 && graph[&start].contains(&(0, target)) {
        print!("[{start}]");
        return start as i32;
    }
    if layers_to_use.len() == 0 {
        return -1;
    }

    for (layer, next) in graph[&start].iter() {
        if layers_to_use.contains(&layer) && !used.contains(&next) {
            let res = dfs_sum(*next, target, graph, [used.clone(), vec![*next]].concat(), layers_to_use.iter().filter(|&&l| l != *layer).map(|&x| x).collect_vec());
            if res >= 0 {
                print!("{next} ");
                return res + start as i32;
            }
        }
    }
    -1
}

fn main() {
    let mut graph: HashMap<u32, HashSet<(usize, u32)>> = HashMap::new();
    let gonal: Vec<Vec<u32>> = [
        |n: u32| n * (n + 1) / 2,
        |n: u32| n * n,
        |n: u32| n * (3 * n - 1) / 2,
        |n: u32| n * (2 * n - 1),
        |n: u32| n * (5 * n - 3) / 2,
        |n: u32| n * (3 * n - 2),
        ].iter().map(
            |f| (1..1000 as u32).map(f).filter(|&x| x >= 1000 && x < 10000).collect::<Vec<u32>>()
        ).collect::<Vec<Vec<u32>>>();
    
    for &i in gonal.iter().flatten() {
        for (k, vj) in gonal.iter().enumerate() {
            for &j in vj {
                if is_connected(i, j) {
                    graph.entry(i).or_insert(HashSet::new()).insert((k, j));
                }
            }
        }
        // println!("{i} -> {:?}", graph.get(&i));
    }

    for &start in gonal[0].iter() {
        let res = dfs_sum(start, start, &graph, vec![start], (1..6).collect_vec());
        if res > -1 {
            print!("{start}");
            println!(" -> {}", res);
        }
    }
    println!("Done.");

}