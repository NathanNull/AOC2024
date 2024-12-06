use std::collections::HashMap;

pub fn main(input: String, is_pt1: bool) {
    if is_pt1 {
        pt1(input)
    } else {
        pt2(input);
    }
}

// Input is two lists of numbers, side by side. Pair the numbers in order of size,
// and find the difference between each pair. Return the sum.
// Terrible inefficient nonsense but it works
fn pt1(input: String) {
    let lines = input.split("\r\n").map(|l| {
        l.split("   ")
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    });
    let [arr1, arr2] = lines
        .fold([vec![], vec![]], |mut curr, l| {
            curr[0].extend([l[0]]);
            curr[1].extend([l[1]]);
            curr
        })
        .map(|a| a.into_iter().enumerate().collect::<Vec<_>>())
        .map(|mut arr| {
            arr.sort_by_key(|(_, n)| *n);
            arr.into_iter().map(|(_, i)| i).collect::<Vec<_>>()
        });
    let sum = arr1.iter().enumerate().fold(0, |acc, (idx, &pos)| {
        acc + (pos as i64 - arr2[idx] as i64).abs()
    });
    println!("Sum is {sum}");
}

fn pt2(input: String) {
    let lines = input.split("\r\n").map(|l| {
        l.split("   ")
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    });
    let [arr1, arr2] = lines.fold([vec![], vec![]], |mut curr, l| {
        curr[0].extend([l[0]]);
        curr[1].extend([l[1]]);
        curr
    });
    let mut freq = HashMap::new();
    for i in arr2 {
        if !freq.contains_key(&i) {
            freq.insert(i, 1);
        } else {
            *freq.get_mut(&i).unwrap() += 1;
        }
    }

    let score = arr1
        .iter()
        .fold(0, |acc, v| acc + (v * freq.get(&v).unwrap_or(&0)));
    println!("Score: {score}");
}
