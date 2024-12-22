use std::collections::HashMap;

pub fn main(input: String, pt1: bool) {
    let nums = input.lines().map(|l| l.parse::<i64>().unwrap());

    let mut sum = 0;
    let mut overall_sales = HashMap::new();
    for mut num in nums {
        let mut possible_sales = HashMap::new();
        let mut last_price = num % 10;
        let mut diffs = [None, None, None, None];
        for _ in 1..=2000 {
            num = rand_step(num);
            let price = num % 10;
            let diff = price - last_price;
            last_price = price;
            diffs = [diffs[1], diffs[2], diffs[3], Some(diff)];
            if diffs.iter().all(|o| o.is_some()) {
                let k = diffs.map(|d| d.unwrap());
                if !possible_sales.contains_key(&k) {
                    possible_sales.insert(k, price);
                }
            }
        }
        sum += num;
        for (k, v) in possible_sales {
            if !overall_sales.contains_key(&k) {
                overall_sales.insert(k, v);
            } else {
                *overall_sales.get_mut(&k).unwrap() += v;
            }
        }
    }
    if pt1 {
        println!("Sum is {sum}");
    } else {
        let max_p = overall_sales.iter().max_by_key(|(_, b)| **b).unwrap().0;
        println!("Max is {max_p:?}, earning {}", overall_sales.get(max_p).unwrap());
    }
}

fn rand_step(mut input: i64) -> i64 {
    input ^= input * 64;
    input %= 16777216;
    input ^= input / 32;
    input %= 16777216;
    input ^= input * 2048;
    input %= 16777216;
    input
}
