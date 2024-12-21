use std::collections::HashMap;

use itertools::Itertools;

pub fn main(input: String, pt1: bool) {
    let codes = input.lines().collect::<Vec<_>>();
    let mut total_complexity = 0;
    for code in codes {
        let num_part = code.chars().filter(|c| *c != 'A').fold(0, |acc, n| {
            acc * 10 + n.to_string().parse::<usize>().unwrap()
        });
        println!("Num is {num_part}");
        let seq_len = find_dirs(
            code.chars().collect(),
            if pt1 { 3 } else { 26 },
            true,
            &mut HashMap::new(),
        );
        println!("Len is {}\n", seq_len);
        total_complexity += seq_len * num_part;
    }
    println!("Total complexity is {total_complexity}");
}

fn find_dirs(
    seq: Vec<char>,
    num_robots: usize,
    first: bool,
    cache: &mut HashMap<(Vec<char>, usize), usize>,
) -> usize {
    if num_robots == 0 {
        return seq.len();
    }
    if let Some(cached) = cache.get(&(seq.clone(), num_robots)) {
        return *cached;
    }
    let pos_by_char: HashMap<char, (i64, i64)> = HashMap::from_iter([
        ('0', (1, 3)),
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0)),
        ('A', (2, 3)),
        ('^', (1, 0)),
        ('a', (2, 0)),
        ('<', (0, 1)),
        ('v', (1, 1)),
        ('>', (2, 1)),
    ]);
    let mut curr_len = 0;
    let mut curr_pos = if first { (2, 3) } else { (2, 0) };
    for char in &seq {
        let target_pos = pos_by_char.get(&char).unwrap();
        let mut best_len = usize::MAX;
        let possibilities =
            posns_to_dirs(&curr_pos, target_pos, if first { (0, 3) } else { (0, 0) });
        for possibility in possibilities {
            let len = find_dirs(possibility.clone(), num_robots - 1, false, cache);
            if len < best_len {
                best_len = len;
            }
        }
        if best_len == usize::MAX {
            println!("Going from {curr_pos:?} to {target_pos:?} has no possibilities");
            best_len = 1;
        }
        curr_len += best_len;
        curr_pos = *target_pos;
    }
    cache.insert((seq, num_robots), curr_len);
    curr_len
}

fn posns_to_dirs(
    curr_pos: &(i64, i64),
    target_pos: &(i64, i64),
    invalid_space: (i64, i64),
) -> Vec<Vec<char>> {
    let mut dirs = vec![];
    if target_pos.1 > curr_pos.1 {
        dirs.extend(vec!['v'; (target_pos.1 - curr_pos.1) as usize])
    }
    if target_pos.1 < curr_pos.1 {
        dirs.extend(vec!['^'; (curr_pos.1 - target_pos.1) as usize])
    }
    if target_pos.0 > curr_pos.0 {
        dirs.extend(vec!['>'; (target_pos.0 - curr_pos.0) as usize])
    }
    if target_pos.0 < curr_pos.0 {
        dirs.extend(vec!['<'; (curr_pos.0 - target_pos.0) as usize])
    }

    let len = dirs.len();
    return dirs
        .into_iter()
        .permutations(len)
        .unique()
        .filter(|d| {
            let mut pos = *curr_pos;
            for dc in d {
                match dc {
                    '^' => pos.1 -= 1,
                    'v' => pos.1 += 1,
                    '<' => pos.0 -= 1,
                    '>' => pos.0 += 1,
                    _ => unreachable!(),
                }
                if pos == invalid_space {
                    return false;
                }
            }
            true
        })
        .map(|p| p.into_iter().chain(['a']).collect())
        .collect();
}
