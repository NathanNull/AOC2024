use std::{
    collections::{HashSet, VecDeque},
    usize,
};

use crate::helpers::Direction;

pub fn main(input: String, pt1: bool) {
    let base_grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let grid = base_grid
        .iter()
        .map(|l| l.iter().map(|c| *c != '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let start_pos = base_grid
        .iter()
        .enumerate()
        .map(|(y, l)| l.iter().enumerate().map(move |(x, c)| (x, y, c)))
        .flatten()
        .find(|(_, _, c)| **c == 'S')
        .map(|(x, y, _)| (x as i64, y as i64))
        .unwrap();
    let end_pos = base_grid
        .iter()
        .enumerate()
        .map(|(y, l)| l.iter().enumerate().map(move |(x, c)| (x, y, c)))
        .flatten()
        .find(|(_, _, c)| **c == 'E')
        .map(|(x, y, _)| (x as i64, y as i64))
        .unwrap();

    let base_path = find_path(&grid, start_pos, end_pos);
    println!("Found path");
    let pos_grid = base_grid
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .map(|(x, _)| {
                    base_path
                        .iter()
                        .enumerate()
                        .find(move |(_, p)| **p == (x as i64, y as i64))
                        .map(|(i, _)| i)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut num_cuts = 0;
    let max_cheats = if pt1 { 2 } else { 20 };
    for (y, row) in pos_grid.iter().enumerate() {
        for (x, oidx) in row.iter().enumerate() {
            if let Some(idx) = oidx {
                for v_off in 0..=max_cheats {
                    for h_off in 0..=max_cheats - v_off {
                        for xdir in [-1, 1] {
                            if xdir == -1 && h_off == 0 {
                                continue;
                            }
                            for ydir in [-1, 1] {
                                if ydir == -1 && v_off == 0 {
                                    continue;
                                }
                                let new_pos =
                                    (x as i64 + (h_off * xdir), y as i64 + (v_off * ydir));
                                if new_pos.0 >= 0
                                    && new_pos.1 >= 0
                                    && pos_grid
                                        .get(new_pos.1 as usize)
                                        .and_then(|row| row.get(new_pos.0 as usize))
                                        .is_some_and(|po| {
                                            po.is_some_and(|p| {
                                                p >= *idx
                                                    + v_off.abs() as usize
                                                    + h_off.abs() as usize
                                                    + 100
                                            })
                                        })
                                {
                                    num_cuts += 1;
                                    // println!(
                                    //     "Cut from {:?} to {:?} works ({}->{})",
                                    //     (x, y),
                                    //     new_pos,
                                    //     idx,
                                    //     pos_grid[new_pos.1 as usize][new_pos.0 as usize].unwrap()
                                    // );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Number of cuts that work: {num_cuts}");
}

fn find_path(grid: &Vec<Vec<bool>>, start_pos: (i64, i64), end_pos: (i64, i64)) -> Vec<(i64, i64)> {
    let mut seen = HashSet::new();
    let mut to_search = VecDeque::new();
    to_search.push_back((start_pos, vec![]));
    seen.insert(start_pos);
    while !to_search.is_empty() {
        let (pos, hist) = to_search.pop_front().unwrap();
        let mut new_hist = hist.clone();
        new_hist.push(pos);
        for dir in Direction::iter() {
            let offset = dir.offset();
            let new_pos = (pos.0 + offset.0, pos.1 + offset.1);
            if !seen.contains(&new_pos)
                && new_pos.0 >= 0
                && new_pos.1 >= 0
                && grid
                    .get(new_pos.1 as usize)
                    .and_then(|r| r.get(new_pos.0 as usize))
                    .is_some_and(|p| *p)
            {
                seen.insert(new_pos);
                to_search.push_back((new_pos, new_hist.clone()));
                if new_pos == end_pos {
                    new_hist.push(new_pos);
                    return new_hist;
                }
            }
        }
    }
    return vec![];
}
