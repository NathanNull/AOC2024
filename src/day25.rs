use std::collections::HashSet;

pub fn main(input: String, pt1: bool) {
    if !pt1 {
        println!("Click the button, what are you reading this for")
    }
    let mut keys = HashSet::new();
    let mut locks = HashSet::new();
    for diagram in input.split("\r\n\r\n") {
        let lines = diagram.lines().collect::<Vec<_>>();
        let vals = (0..5)
            .map(|n| {
                lines
                    .iter()
                    .filter(|l| l.chars().nth(n).unwrap() == '#')
                    .count()
                    - 1
            })
            .collect::<Vec<_>>();
        if diagram.chars().nth(0).unwrap() == '#' {
            // It's a lock
            locks.insert(vals);
        } else {
            keys.insert(vals);
        }
    }

    println!("{}/{}", locks.len(), keys.len());
    let mut valid_pairs = 0;
    for lock in locks {
        for key in &keys {
            if lock.iter().zip(key.iter()).all(|(l, k)| l + k <= 5) {
                valid_pairs += 1;
                //println!("{lock:?}/{key:?} is valid")
            } else {
                //println!("{lock:?}/{key:?} is invalid")
            }
        }
    }
    println!("{valid_pairs} valid pairs");
}
