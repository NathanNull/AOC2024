use std::collections::{HashMap, HashSet};

use crate::helpers::first_n;

pub fn main(input: String, pt1: bool) {
    let [reqs_strs, updates_strs] =
        first_n(&mut input.split("\r\n\r\n")).map(|s| s.split("\r\n").collect::<Vec<_>>());
    let reqs = reqs_strs
        .into_iter()
        .map(|s| first_n::<2, _>(&mut s.split("|").map(|n| n.parse::<i64>().unwrap())))
        .collect::<Vec<_>>();
    let updates = updates_strs
        .into_iter()
        .map(|s| {
            s.split(",")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut reqs_dict: HashMap<i64, Vec<i64>> = HashMap::new();
    for [first, second] in reqs {
        if !reqs_dict.contains_key(&second) {
            reqs_dict.insert(second, vec![]);
        }
        reqs_dict.get_mut(&second).unwrap().push(first);
    }

    let mut sum = 0;
    for update in updates {
        if is_valid(&reqs_dict, &update) {
            // If it's pt1 we're good
            sum += update[update.len() / 2] * if pt1 { 1 } else { 0 };
        } else if !pt1 {
            let mut new_update = vec![];
            for page in update.iter() {
                // If we can add it, we just do
                new_update.push(*page);
                if !is_valid(&reqs_dict, &new_update) {
                    // If not, we check if we can add it progressively farther back
                    new_update.pop();
                    let mut added = false;
                    for i in (0..new_update.len()).rev() {
                        new_update.insert(i, *page);
                        added = true;
                        if is_valid(&reqs_dict, &new_update) {
                            break;
                        }
                        new_update.remove(i);
                        added = false;
                    }
                    // If we never can, I might have screwed something up somewhere
                    // This doesn't seem to happen though
                    if !added {
                        panic!("Something has gone wrong: {update:?}|||{new_update:?}");
                    }
                }
            }
            println!("Corrected {update:?} to {new_update:?}");
            sum += new_update[new_update.len() / 2];
        }
    }
    println!("Sum is {sum}");
}

fn is_valid(reqs: &HashMap<i64, Vec<i64>>, update: &Vec<i64>) -> bool {
    let mut disallowed: HashSet<i64> = HashSet::new();
    let mut valid = true;
    for page in update.iter() {
        if disallowed.contains(&page) {
            valid = false;
            break;
        }
        disallowed.extend(reqs.get(&page).unwrap_or(&vec![]));
    }
    return valid;
}
