use std::{collections::HashSet, i64};

use crate::helpers::{first_n, Direction};

pub fn main(input: String, pt1: bool) {
    const EXAMPLE: bool = false;
    let gridsize = if EXAMPLE { 6 } else { 70 };

    if pt1 {
        let num_bytes = if EXAMPLE { 12 } else { 1024 };
        let bytes: HashSet<[i64; 2]> = HashSet::from_iter(
            input
                .lines()
                .take(num_bytes)
                .map(|b| first_n::<2, _>(&mut b.split(",").map(|n| n.parse::<i64>().unwrap()))),
        );
        println!(
            "Min steps is {}",
            find_path(gridsize, &bytes).unwrap().len()
        );
    } else {
        let mut bytes = HashSet::new();
        let mut curr_path: HashSet<(i64, i64)> = HashSet::new();
        for line in input.lines() {
            let byte = first_n::<2, _>(&mut line.split(",").map(|n| n.parse::<i64>().unwrap()));
            bytes.insert(byte);
            // If our last path wasn't blocked by the new byte, no need to check if one exists because clearly one does
            if !curr_path.is_empty() && !curr_path.iter().any(|p| p.0 == byte[0] && p.1 == byte[1])
            {
                continue;
            }
            if let Some(path) = find_path(gridsize, &bytes) {
                curr_path = path;
            } else {
                println!("Byte {byte:?} broke the path");
                break;
            }
        }
    }
}

fn find_path(gridsize: i64, bytes: &HashSet<[i64; 2]>) -> Option<HashSet<(i64, i64)>> {
    let mut to_search = vec![((0, 0), HashSet::from_iter([(0, 0)]))];
    let mut seen = HashSet::new();
    seen.insert((0, 0));
    while !to_search.is_empty() {
        to_search.sort_by_key(|(_, s)| -(s.len() as i64));
        let (pos, steps) = to_search.pop().unwrap();
        for dir in Direction::iter() {
            let offset = dir.offset();
            let new_pos = (pos.0 + offset.0, pos.1 + offset.1);
            if new_pos.0 >= 0
                && new_pos.1 >= 0
                && new_pos.0 <= gridsize
                && new_pos.1 <= gridsize
                && !bytes.contains(&[new_pos.0, new_pos.1])
                && !seen.contains(&new_pos)
            {
                seen.insert(new_pos);
                let mut new_set = steps.clone();
                new_set.insert(new_pos);
                if new_pos == (gridsize, gridsize) {
                    return Some(new_set);
                } else {
                    to_search.push((new_pos, new_set));
                }
            }
        }
    }
    return None;
}
