use std::collections::{HashSet, VecDeque};

pub fn main(input: String, pt1: bool) {
    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut seen = HashSet::new();
    let mut plots = vec![];
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if seen.contains(&(x as i64, y as i64)) {
                continue;
            }
            let (region, perimeter, sides) = floodfill(&grid, (x, y), *cell);
            plots.push((cell, region.len() * perimeter, region.len() * sides));
            seen.extend(region);
        }
    }
    println!(
        "Total price is {}",
        plots
            .iter()
            .fold(0, |acc, (_, v, v2)| acc + (if pt1 { v } else { v2 }))
    );
}

fn floodfill(
    grid: &Vec<Vec<char>>,
    pos: (usize, usize),
    cell: char,
) -> (HashSet<(i64, i64)>, usize, usize) {
    let mut seen = HashSet::new();
    let mut edges = HashSet::new();
    seen.insert((pos.0 as i64, pos.1 as i64));
    let mut to_search = VecDeque::new();
    let mut perimeter = 0;
    to_search.push_back((pos.0 as i64, pos.1 as i64));
    while !to_search.is_empty() {
        let test_pos = to_search.pop_front().unwrap();
        const OFFSETS: [(i64, i64); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut non_neighbours: HashSet<(i64, i64)> = HashSet::from_iter(OFFSETS);
        for offset in OFFSETS {
            let new_pos = (test_pos.0 + offset.0, test_pos.1 + offset.1);
            if new_pos.0 >= 0
                && new_pos.1 >= 0
                && grid
                    .get(new_pos.1 as usize)
                    .and_then(|r| r.get(new_pos.0 as usize))
                    .is_some_and(|c| *c == cell)
            {
                non_neighbours.remove(&offset);
                if !seen.contains(&new_pos) {
                    seen.insert(new_pos);
                    to_search.push_back(new_pos);
                }
            }
        }
        perimeter += non_neighbours.len();
        if !non_neighbours.is_empty() {
            edges.extend(non_neighbours.into_iter().map(|n| (test_pos, n)));
        }
    }
    let mut sides: HashSet<((i64, i64), Vec<(i64, i64)>)> =
        HashSet::from_iter(edges.into_iter().map(|e| (e.1, vec![e.0])));
    let mut had_effect = true;
    while had_effect {
        had_effect = false;
        let mut new_sides: HashSet<((i64, i64), Vec<(i64, i64)>)> = HashSet::new();
        for side in sides.iter() {
            if let Some(e_ref) = new_sides.iter().find(|e| {
                e.0 == side.0
                    && e.1
                        .iter()
                        .find(|p| {
                            side.1
                                .iter()
                                .find(|q| (q.0 - p.0).abs() + (q.1 - p.1).abs() == 1)
                                .is_some()
                        })
                        .is_some()
            }) {
                let e = e_ref.clone();
                new_sides.remove(&e);
                let mut new_side = e.1.clone();
                new_side.extend(side.1.clone());
                //println!("Collapsed {side:?} and {e:?} into {new_side:?}");
                new_sides.insert((
                    e.0,
                    HashSet::<(i64, i64)>::from_iter(new_side)
                        .into_iter()
                        .collect(),
                ));
                had_effect = true;
            } else {
                new_sides.insert(side.clone());
            }
        }
        sides = new_sides;
    }
    return (seen, perimeter, sides.len());
}
