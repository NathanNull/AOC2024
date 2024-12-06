use std::collections::{HashMap, HashSet};

pub fn main(input: String, pt1: bool) {
    let grid_chars = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let guard = {
        let (row_i, row) = grid_chars
            .iter()
            .enumerate()
            .find(|(_, row)| row.contains(&'^'))
            .unwrap();
        let (col, _) = row.iter().enumerate().find(|(_, c)| **c == '^').unwrap();
        (row_i as i64, col as i64, -1, 0)
    };
    let mut grid = grid_chars
        .iter()
        .map(|r| r.iter().map(|c| *c == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let visited = try_path(&grid, guard).unwrap();
    if pt1 {
        println!("Visited {} tiles", visited.len());
    } else {
        let curr_guard = {
            let (row_i, row) = grid_chars
                .iter()
                .enumerate()
                .find(|(_, row)| row.contains(&'^'))
                .unwrap();
            let (col, _) = row.iter().enumerate().find(|(_, c)| **c == '^').unwrap();
            (row_i as i64, col as i64, -1, 0)
        };
        let mut count = 0;
        for cell in visited {
            let g = grid.get_mut(cell.0.0 as usize).unwrap()[cell.0.1 as usize];
            grid.get_mut(cell.0.0 as usize).unwrap()[cell.0.1 as usize] = true;
            if try_path(&grid, curr_guard).is_none() {
                count += 1;
                if count % 200 == 0 {
                    println!("{count} found")
                }
            }
            grid.get_mut(cell.0.0 as usize).unwrap()[cell.0.1 as usize] = g;
        }
        println!(
            "Could block in {} places",
            count
        );
    }
}

fn try_path(
    grid: &Vec<Vec<bool>>,
    curr_guard: (i64, i64, i64, i64),
) -> Option<HashMap<(i64, i64), HashSet<(i64, i64)>>> {
    let mut guard = curr_guard.clone();
    let mut visited: HashMap<(i64, i64), HashSet<(i64, i64)>> = HashMap::new();
    while guard.0 > 0
        && guard.1 > 0
        && guard.0 < grid.len() as i64
        && guard.1 < grid[0].len() as i64
    {
        let new_pos = (guard.0 + guard.2, guard.1 + guard.3);
        if new_pos.0 < 0 || new_pos.1 < 0 {
            guard.0 = -1;
        } else if let Some(obstacle) = grid
            .get(new_pos.0 as usize)
            .and_then(|r| r.get(new_pos.1 as usize))
        {
            if *obstacle {
                // Turn right, I think
                let new_dir = (guard.3, -guard.2);
                guard.2 = new_dir.0;
                guard.3 = new_dir.1;
            } else {
                guard.0 = new_pos.0;
                guard.1 = new_pos.1;
                if !visited.contains_key(&(guard.0, guard.1)) {
                    visited.insert((guard.0, guard.1), HashSet::new());
                }
                if visited
                    .get(&(guard.0, guard.1))
                    .unwrap()
                    .contains(&(guard.2, guard.3))
                {
                    return None;
                }
                visited
                    .get_mut(&(guard.0, guard.1))
                    .unwrap()
                    .insert((guard.2, guard.3));
            }
        } else {
            guard.0 = -1;
        }
    }
    return Some(visited);
}
