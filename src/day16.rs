use std::collections::{HashMap, HashSet};

use crate::helpers::Direction;

pub fn main(input: String, pt1: bool) {
    let base_grid = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| (x, y, c))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let maze = base_grid
        .iter()
        .map(|l| l.iter().map(|(_, _, c)| *c != '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let start = base_grid
        .iter()
        .flatten()
        .find(|(_, _, c)| *c == 'S')
        .map(|(x, y, _)| (*x as i64, *y as i64))
        .unwrap();
    let end = base_grid
        .iter()
        .flatten()
        .find(|(_, _, c)| *c == 'E')
        .map(|(x, y, _)| (*x as i64, *y as i64))
        .unwrap();
    let mut to_explore: Vec<((i64, i64), Direction, usize, Vec<(i64, i64)>)> = vec![];
    let mut seen = HashMap::new();
    let mut min_score = usize::MAX;
    let mut min_hist = vec![];
    seen.insert((start, Direction::Right), 0);
    to_explore.push((start, Direction::Right, 0, vec![]));
    while !to_explore.is_empty() {
        to_explore.sort_by_key(|(_, _, s, _)| -(*s as i64));
        let (pos, dir, score, hist) = to_explore.pop().unwrap();
        for next_dir in Direction::iter() {
            let offset = next_dir.offset();
            let new_pos = (pos.0 + offset.0, pos.1 + offset.1);
            let new_score = score
                + if dir == next_dir {
                    1
                } else if dir == next_dir.opposite() {
                    2001
                } else {
                    1001
                };
            let new_hist: Vec<(i64, i64)> = hist.clone().into_iter().chain([pos]).collect();
            if maze[new_pos.1 as usize][new_pos.0 as usize]
                && (!seen.contains_key(&(new_pos, next_dir))
                    || seen.get(&(new_pos, next_dir)).unwrap() >= &new_score)
            {
                if !to_explore
                    .iter()
                    .any(|(p, d, s, _)| *p == new_pos && *d == next_dir && *s <= new_score)
                {
                    to_explore.push((new_pos, next_dir, new_score, new_hist.clone()));
                    seen.insert((new_pos, next_dir), new_score);
                } else if !to_explore
                    .iter()
                    .any(|(p, d, s, _)| *p == new_pos && *d == next_dir && *s < new_score)
                {
                    for (p, d, s, h) in to_explore.iter_mut() {
                        if *p == new_pos && *d == next_dir && *s == new_score {
                            *h = HashSet::<_>::from_iter(h.clone().into_iter().chain(new_hist.clone()))
                                .iter()
                                .map(|info| *info)
                                .collect();
                        }
                    }
                }
            }
            if new_pos == end {
                println!("Solution found with score {new_score}");
                if new_score < min_score {
                    min_score = new_score;
                    min_hist = vec![new_hist.clone()];
                } else if new_score == min_score {
                    min_hist.push(new_hist.clone());
                }
            }
        }
    }
    if pt1 {
        println!("Min score is {min_score}");
    } else {
        let tile_set: HashSet<(i64, i64)> = HashSet::from_iter(min_hist.into_iter().flatten());
        println!("Tiles reached are {}", tile_set.len() + 1); // I think we're missing the end tile or something
    }
}
