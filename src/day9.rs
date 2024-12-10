use std::collections::VecDeque;

pub fn main(input: String, pt1: bool) {
    // is filled, id, length
    let mut orig_disk = input
        .chars()
        .enumerate()
        .map(|(i, n)| (i % 2 == 0, i / 2, n.to_string().parse::<i64>().unwrap()))
        .collect::<VecDeque<_>>();
    //println!("Orig disk is {orig_disk:?}");
    let mut new_disk = vec![];
    if pt1 {
        while !orig_disk.is_empty() {
            let next = orig_disk.pop_front().unwrap();
            match next {
                (true, id, len) => {
                    println!("Appending {id}*{len}");
                    for _ in 0..len {
                        new_disk.push(id);
                    }
                }
                (false, _, mut len) => {
                    while len > 0 {
                        //println!("Filling void, curr disk is {orig_disk:?}");
                        let mut last = orig_disk.pop_back().unwrap();
                        orig_disk.pop_back(); // Get rid of the void before it
                        assert!(last.0);
                        while last.2 > 0 && len > 0 {
                            last.2 -= 1;
                            len -= 1;
                            new_disk.push(last.1);
                        }
                        if last.2 != 0 {
                            // Need to push a void first to keep alternating pattern
                            orig_disk.push_back((false, 0, 0));
                            orig_disk.push_back(last);
                        }
                    }
                }
            }
        }
    } else {
        let mut index_to_remove = orig_disk.back().unwrap().1;
        while index_to_remove > 0 {
            let idx_of_file = orig_disk
                .iter()
                .enumerate()
                .find(|(_, (_, i, _))| *i == index_to_remove)
                .unwrap()
                .0;
            let last = orig_disk.remove(idx_of_file).unwrap();
            orig_disk.insert(idx_of_file, (false, 0, last.2));
            //println!("Testing {last:?} (disk is {:?})", simplify_disk(&orig_disk));
            let mut to_prepend = vec![];
            let mut placed = false;
            while !orig_disk.is_empty() {
                let next = orig_disk.pop_front().unwrap();
                match next.0 {
                    false => {
                        if next.2 >= last.2 {
                            //println!("Found position {next:?} (disk is {:?}", simplify_disk(&orig_disk));
                            to_prepend.push(last);
                            if next.2 > last.2 {
                                to_prepend.push((false, 0, next.2 - last.2));
                            }
                            placed = true;
                            break;
                        } else {
                            to_prepend.push(next);
                        }
                    }
                    true => to_prepend.push(next),
                }
            }
            if !placed {
                to_prepend.push(last);
            }
            for rep in to_prepend.into_iter().rev() {
                orig_disk.push_front(rep);
            }
            index_to_remove -= 1;
            //println!("Done {last:?} (disk is {:?})", simplify_disk(&orig_disk));
        }
        for f in orig_disk {
            for _ in 0..f.2 {
                new_disk.push(if f.0 { f.1 } else { 0 })
            }
        }
    }
    //println!("Final disk is {new_disk:?}");
    let checksum = new_disk
        .iter()
        .enumerate()
        .fold(0, |acc, (i, id)| acc + (i * id));
    println!("Checksum: {checksum}");
}