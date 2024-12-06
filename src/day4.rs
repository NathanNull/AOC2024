pub fn main(input: String, is_pt1: bool) {
    if is_pt1 {
        pt1(input)
    } else {
        pt2(input);
    }
}

fn pt1(input: String) {
    let grid: Vec<Vec<char>> = input
        .split("\r\n")
        .map(|line| line.chars().collect())
        .collect();
    let mut xmas_count = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'X' {
                for ox in -1i64..=1 {
                    for oy in -1i64..=1 {
                        // We actually don't need to manually rule out ox=0,oy=0 because how would that spell XMAS
                        let mut valid = true;
                        for (n, letter) in ['X', 'M', 'A', 'S'].into_iter().enumerate().skip(1) {
                            // We can skip the first one because obviously the cell offset by 0 is an X
                            let (px, py) = (x as i64 + ox * n as i64, y as i64 + oy * n as i64);
                            if px < 0 || py < 0 {
                                continue;
                            }
                            if !grid
                                .get(py as usize)
                                .and_then(|r| r.get(px as usize))
                                .is_some_and(|l| *l == letter)
                            {
                                valid = false;
                                break;
                            }
                        }
                        if valid {
                            xmas_count += 1;
                        }
                    }
                }
            }
        }
    }
    println!("XMAS count is {xmas_count}");
}

fn pt2(input: String) {
    let grid: Vec<Vec<char>> = input
        .split("\r\n")
        .map(|line| line.chars().collect())
        .collect();
    let mut xmas_count = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'A' {
                let mut letters = vec![];
                for ox in -1i64..=1 {
                    for oy in -1i64..=1 {
                        let (px, py) = (x as i64 + ox, y as i64 + oy);
                        if px < 0 || py < 0 {
                            continue;
                        }
                        if ox * oy == 0 {
                            continue;
                        }
                        // Rust option gaming
                        if let Some(letter) =
                            grid.get(py as usize).and_then(|line| line.get(px as usize))
                        {
                            letters.push(*letter);
                        }
                    }
                }
                if letters.iter().filter(|l| **l == 'M').count() == 2
                    && letters.iter().filter(|l| **l == 'S').count() == 2
                    && letters[3] != letters[0] // Rules out the case where you have MAM/SAS
                {
                    xmas_count += 1;
                }
            }
        }
    }
    println!("XMAS count is {xmas_count}");
}
