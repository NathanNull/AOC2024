use std::collections::{HashMap, HashSet, VecDeque};

use crate::helpers::first_n;

pub fn main(input: String, pt1: bool) {
    let [towels_str, patterns_str] = first_n(&mut input.split("\r\n\r\n"));
    let towels = towels_str.split(", ").collect::<HashSet<_>>();
    let patterns = patterns_str
        .lines()
        .map(|l| l.chars().collect::<VecDeque<_>>())
        .collect::<Vec<_>>();
    let mut patterns_possible = 0;
    for pattern in patterns {
        print!(
            "Pattern {} ",
            pattern
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join("")
        );
        let ways = possible(pattern, &towels, 0, &mut HashMap::new());
        if pt1 {
            if ways != 0 {
                patterns_possible += 1;
                println!("works");
            } else {
                println!("is invalid");
            }
        } else {
            println!(" has {ways} patterns");
            patterns_possible += ways;
        }
    }
    println!("Patterns possible: {patterns_possible}");
}

fn possible(
    mut pattern: VecDeque<char>,
    towels: &HashSet<&str>,
    depth: usize,
    cache: &mut HashMap<VecDeque<char>, usize>,
) -> usize {
    if pattern.len() == 0 {
        return 1;
    }
    if cache.contains_key(&pattern) {
        return *cache.get(&pattern).unwrap();
    }
    let base_pattern = pattern.clone();
    assert!(depth < 60);
    let mut curr_string = "".to_string();
    let mut to_try = vec![];
    while pattern.len() > 0 {
        curr_string += &pattern.pop_front().unwrap().to_string();
        if towels.contains(&curr_string.as_str()) {
            to_try.push(pattern.clone());
        }
    }
    to_try.sort_by_key(|v| -(v.len() as i64));
    let mut ret = 0;
    for pat in to_try {
        ret += possible(pat, towels, depth + 1, cache);
    }
    cache.insert(base_pattern, ret);
    return ret;
}
