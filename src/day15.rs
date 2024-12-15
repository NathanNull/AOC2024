use std::collections::HashSet;

use crate::helpers::first_n;

pub fn main(input: String, pt1: bool) {
    let [grid_i, dir_i] = first_n(&mut input.split("\r\n\r\n"));
    let mut grid = grid_i
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    if !pt1 {
        let old_g = grid.iter().map(|l| l.clone()).collect::<Vec<_>>();
        grid = vec![];
        for line in old_g {
            grid.push(vec![]);
            for char in line {
                grid.last_mut().unwrap().extend(match char {
                    'O' => ['[', ']'],
                    '@' => ['@', '.'],
                    a => [a, a],
                });
            }
        }
    }
    // println!(
    //     "Grid is {}",
    //     grid.iter()
    //         .map(|l| format!("{:?}", l))
    //         .collect::<Vec<_>>()
    //         .join("\n")
    // );
    let dirs = dir_i
        .lines()
        .collect::<Vec<_>>()
        .join("")
        .chars()
        .collect::<Vec<_>>();
    let mut robot = (0, 0);
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '@' {
                robot = (x as i64, y as i64);
                break;
            }
        }
        if robot != (0, 0) {
            break;
        }
    }
    grid[robot.1 as usize][robot.0 as usize] = '.';
    for dir in dirs {
        //println!("Dir is {dir}");
        let offset = match dir {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            a => panic!("Invalid direction '{a}'"),
        };
        let new_pos = (robot.0 + offset.0, robot.1 + offset.1);
        match grid[new_pos.1 as usize][new_pos.0 as usize] {
            '.' => robot = new_pos,
            '#' => (),
            // A horizontal push on a big box is the same as a big box push
            x if ['[', ']', 'O'].contains(&x) && (offset.1 == 0 || x == 'O') => {
                let mut new_box_pos = new_pos;
                while ['[', ']', 'O']
                    .contains(&grid[new_box_pos.1 as usize][new_box_pos.0 as usize])
                {
                    new_box_pos = (new_box_pos.0 + offset.0, new_box_pos.1 + offset.1);
                }
                if grid[new_box_pos.1 as usize][new_box_pos.0 as usize] == '.' {
                    let mut move_pos = new_pos;
                    let mut cur_held = '.';
                    while move_pos != new_box_pos {
                        let temp = cur_held;
                        cur_held = grid[move_pos.1 as usize][move_pos.0 as usize];
                        grid[move_pos.1 as usize][move_pos.0 as usize] = temp;
                        move_pos = (move_pos.0 + offset.0, move_pos.1 + offset.1);
                    }
                    grid[move_pos.1 as usize][move_pos.0 as usize] = cur_held;
                    robot = new_pos;
                }
            }
            '[' | ']' => {
                if let Some(pushes) = push_vert(&grid, new_pos, offset.1) {
                    let p_chars = pushes
                        .iter()
                        .map(|&p| (p, grid[p.1 as usize][p.0 as usize]))
                        .collect::<Vec<_>>();
                    //println!("Pushing up {p_chars:?}");
                    for (pos, char) in p_chars.iter() {
                        if !p_chars
                            .iter()
                            .any(|((x, y), _)| *x == pos.0 && *y == pos.1 - offset.1)
                        {
                            grid[pos.1 as usize][pos.0 as usize] = '.'
                        }
                        grid[(pos.1 + offset.1) as usize][pos.0 as usize] = *char;
                    }
                    robot = new_pos;
                }
            }
            _ => panic!("Invalid grid character"),
        }
        grid[robot.1 as usize][robot.0 as usize] = '@';
        // println!(
        //     "Grid is \n{}, robot is {robot:?}\n",
        //     grid.iter()
        //         .map(|l| l.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(""))
        //         .collect::<Vec<_>>()
        //         .join("\n")
        // );
        grid[robot.1 as usize][robot.0 as usize] = '.';
        assert!(!grid.iter().any(|l| l.contains(&'O')));
    }
    let mut gps_sum = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'O' || *c == '[' {
                gps_sum += 100 * y + x;
            }
        }
    }
    println!("GPS sum is {gps_sum}");
}

fn push_vert(grid: &Vec<Vec<char>>, pos: (i64, i64), v_offset: i64) -> Option<HashSet<(i64, i64)>> {
    //print!("Testing pos={pos:?}");
    let new_pos = (pos.0, pos.1 + v_offset);
    match grid[pos.1 as usize][pos.0 as usize] {
        x if ['[', ']'].contains(&x) => {
            //println!(" (char is {x} (gridline {:?}))", grid[pos.1 as usize]);
            let side_1 = push_vert(grid, new_pos, v_offset);
            let side_2 = push_vert(
                grid,
                (new_pos.0 + if x == '[' { 1 } else { -1 }, new_pos.1),
                v_offset,
            );
            if let Some(mut pushes) = side_1
                .and_then(|v1| side_2.map(|v2| v1.union(&v2).map(|&p| p).collect::<HashSet<_>>()))
            {
                pushes.insert(pos);
                pushes.insert((pos.0 + if x == '[' { 1 } else { -1 }, pos.1));
                Some(pushes)
            } else {
                None
            }
        }
        '.' => Some(HashSet::new()),
        '#' => None,
        x => panic!("Invalid character {x}"),
    }
}
