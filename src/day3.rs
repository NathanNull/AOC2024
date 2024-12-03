use regex::Regex;

pub fn pt1(input: String) {
    main(input, true);
}

pub fn pt2(input: String) {
    main(input, false);
}

fn main(input: String, pt1: bool) {
    // Matches mul([numbers],[numbers]), or do() with an optional n't in the middle (i.e. don't())
    let muls = Regex::new(r"(mul\((\d*),(\d*)\))|(do(n't)?\(\))").unwrap();
    let num = Regex::new(r"\d+").unwrap();
    let mut sum = 0;
    let mut enabled = true;
    for result in muls.find_iter(&input) {
        let str = result.as_str();
        if str.contains("don't") {
            enabled = pt1; // Only disable if it isn't pt1
        } else if str.contains("do") {
            enabled = true;
        } else if enabled {
            let product = num
                .find_iter(str)
                .map(|s| s.as_str().parse::<i64>().unwrap())
                .fold(1, |acc, n| acc * n);
            sum += product;
        }
    }

    println!("Sum of products is {sum}");
}
