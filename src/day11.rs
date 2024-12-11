use std::collections::HashMap;

pub fn main(input: String, pt1: bool) {
    let mut stones: HashMap<i64, usize> = HashMap::new();
    for s in input.split(" ").map(|n| n.parse::<i64>().unwrap()) {
        if !stones.contains_key(&s) {
            stones.insert(s, 0);
        }
        *stones.get_mut(&s).unwrap() += 1;
    }
    for i in 0..(if pt1 { 25 } else { 75 }) {
        let mut new_stones = vec![];
        for (stone, &count) in stones.iter() {
            let str = stone.to_string();
            if *stone == 0 {
                new_stones.push((1, count));
            } else if str.len() % 2 == 0 {
                let pair = str.split_at(str.len() / 2);
                new_stones.extend([
                    (pair.0.parse::<i64>().unwrap(), count),
                    (pair.1.parse::<i64>().unwrap(), count),
                ]);
            } else {
                new_stones.push((stone * 2024, count));
            }
        }
        stones = HashMap::new();
        for (s, c) in new_stones {
            if !stones.contains_key(&s) {
                stones.insert(s, 0);
            }
            *stones.get_mut(&s).unwrap() += c;
        }
        if i % 5 == 0 {
            println!(
                "Done {i} steps, current len is {}",
                stones.iter().fold(0, |acc, (_, c)| acc + c)
            );
        }
    }
    println!("{} stones", stones.iter().fold(0, |acc, (_, c)| acc + c))
}
