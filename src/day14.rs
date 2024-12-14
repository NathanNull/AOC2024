use std::collections::HashSet;

use crate::helpers::first_n;

pub fn main(input: String, pt1: bool) {
    const IS_EXAMPLE: bool = false;
    let room_size = if IS_EXAMPLE { (11, 7) } else { (101, 103) };
    let mut robots = input
        .lines()
        .map(|l| {
            first_n::<2, _>(&mut l.split(" ").map(|v| {
                first_n::<2, _>(
                    &mut v
                        .split("=")
                        .nth(1)
                        .unwrap()
                        .split(",")
                        .map(|n| n.parse::<i64>().unwrap()),
                )
            }))
        })
        .collect::<Vec<_>>();
    let mut quadrants = [[0, 0], [0, 0]];
    // 10403 determined to be the loop point for my input (probably globally as well as 103*101=10403)
    for i in 0..if pt1 { 100 } else { 10403 } {
        for [pos, vel] in robots.iter_mut() {
            *pos = [
                (pos[0] + vel[0] + room_size.0) % room_size.0,
                (pos[1] + vel[1] + room_size.1) % room_size.1,
            ];
        }
        if !pt1 {
            let pos_set: HashSet<&[i64; 2]> = HashSet::from_iter(robots.iter().map(|[p, _]| p));
            if i<10 || maybe_tree(&pos_set) {
                for x in 0..room_size.0 {
                    for y in 0..room_size.1 {
                        print!("{}", if pos_set.contains(&[x, y]) { '#' } else { '.' });
                    }
                    println!();
                }
                println!("{i}-------------------");
            }
            if i % 1000 == 0 {
                println!("Reached {i} iters");
            }
        }
    }
    if pt1 {
        for [pos, _] in robots {
            let xidx = if pos[0] < room_size.0 / 2 {
                0
            } else if pos[0] > room_size.0 / 2 {
                1
            } else {
                continue;
            };
            let yidx = if pos[1] < room_size.1 / 2 {
                0
            } else if pos[1] > room_size.1 / 2 {
                1
            } else {
                continue;
            };
            quadrants[xidx][yidx] += 1;
        }
        println!(
            "Quadrants are {quadrants:?} for a product of {}",
            quadrants[0][0] * quadrants[0][1] * quadrants[1][0] * quadrants[1][1]
        );
    }
}

fn maybe_tree(posns: &HashSet<&[i64; 2]>) -> bool {
    let mut diag_count = 0;
    for pos in posns.iter() {
        if (posns.contains(&[pos[0] + 1, pos[1] + 1]) && posns.contains(&[pos[0] + 2, pos[1] + 2]))
            || (posns.contains(&[pos[0] - 1, pos[1] + 1])
                && posns.contains(&[pos[0] - 2, pos[1] + 2]))
        {
            diag_count += 1;
        }
    }
    // Random guess
    return diag_count >= 30;
}
