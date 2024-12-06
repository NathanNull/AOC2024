extern crate find_folder;
extern crate regex;

use find_folder::Search;
use std::fs::read_to_string;

const INFO: (usize, usize, usize) = (6,2,0);
const DAY: usize = INFO.0;
const PART: usize = INFO.1;
const IS_TEST: bool = INFO.2 == 1;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod helpers;

fn main() {
    let days = [
        [day1::pt1, day1::pt2],
        [day2::pt1, day2::pt2],
        [day3::pt1, day3::pt2],
        [day4::pt1, day4::pt2],
        [day5::pt1, day5::pt2],
        [day6::pt1, day6::pt2],
    ];
    let assets = Search::ParentsThenKids(3, 3).for_folder("inputs").unwrap();
    let input_path = assets.join(std::format!(
        "day{}{}.txt",
        DAY,
        if IS_TEST { "-test" } else { "" }
    ));
    let input = read_to_string(input_path).unwrap();
    days[DAY - 1][PART - 1](input);
}
