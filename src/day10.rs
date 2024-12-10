use std::collections::HashSet;

pub fn main(input: String, pt1: bool) {
    let grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, n) in line.iter().enumerate() {
            if *n == 0 {
                let score = calculate_score(&grid, x as i64, y as i64, 0, pt1);
                //println!("Score for {:?} is {}", (x, y), score.len());
                sum += score.len();
            }
        }
    }
    println!("Trailhead sum is {sum}");
}

fn calculate_score(
    grid: &Vec<Vec<u8>>,
    x: i64,
    y: i64,
    n: u8,
    allow_dupes: bool,
) -> Vec<(i64, i64)> {
    if n == 9 {
        return vec![(x, y)];
    }
    let mut score = vec![];
    for (ox, oy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let (nx, ny) = (x + ox, y + oy);
        if nx < 0
            || ny < 0
            || *grid
                .get(ny as usize)
                .and_then(|l| l.get(nx as usize))
                .unwrap_or(&n)
                != n + 1
        {
            continue;
        }
        let other_score = calculate_score(grid, nx, ny, n + 1, allow_dupes);
        score.extend(other_score);
    }
    if allow_dupes {
        let hs: HashSet<(i64, i64)> = HashSet::from_iter(score);
        return hs.into_iter().collect();
    } else {
        return score;
    }
}
