use crate::helpers::gcd_of_two_numbers;
use std::collections::HashSet;

pub fn main(input: String, pt1: bool) {
    let chars: HashSet<char> =
        HashSet::from_iter(input.chars().filter(|c| !['\n', '\r', '.'].contains(&c)));
    let g_base: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.chars().map(|_| false).collect())
        .collect();
    let grids = chars
        .into_iter()
        .map(|c| {
            let mut g = vec![];
            for (y, line) in input.lines().enumerate() {
                for (x, cell) in line.chars().enumerate() {
                    if cell == c {
                        g.push((x as i64, y as i64));
                    }
                }
            }
            g
        })
        .collect::<Vec<_>>();

    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();
    for grid in grids {
        let mut grid_antinodes: HashSet<(i64, i64)> = HashSet::new();

        for i in 0..grid.len() - 1 {
            for j in i + 1..grid.len() {
                let gi = grid[i];
                let gj = grid[j];
                let offset = (gi.0 - gj.0, gi.1 - gj.1);
                if pt1 {
                    let a1 = (gj.0 - offset.0, gj.1 - offset.1);
                    let a2 = (gi.0 + offset.0, gi.1 + offset.1);
                    for node in [a1, a2] {
                        if node.0 >= 0
                            && node.1 >= 0
                            && node.0 < g_base[0].len() as i64
                            && node.1 < g_base.len() as i64
                        {
                            grid_antinodes.insert(node);
                        }
                    }
                } else {
                    let offset_gcd = gcd_of_two_numbers(offset.0, offset.1);
                    let slope = (offset.0 / offset_gcd, offset.1 / offset_gcd);
                    let iters_needed = (i64::max(g_base.len() as i64, g_base[0].len() as i64)
                        / i64::min(slope.0, slope.1))
                    .abs() + 1; // +1 just in case (I think there's an off-by-one sometimes due to integer division)
                    for n in -iters_needed..iters_needed {
                        let node = (gj.0 + n * slope.0, gj.1 + n * slope.1);
                        if node.0 >= 0
                            && node.1 >= 0
                            && node.0 < g_base[0].len() as i64
                            && node.1 < g_base.len() as i64
                        {
                            grid_antinodes.insert(node);
                        }
                    }
                }
            }
        }

        antinodes.extend(grid_antinodes);
    }
    println!("There are {} antinodes", antinodes.len());
}
