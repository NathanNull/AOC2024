extern crate find_folder;
extern crate regex;

use find_folder::Search;
use std::fs::read_to_string;

const INFO: (usize, usize, usize) = (9,1,1);
const DAY: usize = INFO.0;
const PART: usize = INFO.1;
const IS_TEST: bool = INFO.2 == 1;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod helpers;

fn main() {
    let days = [
        day1::main,
        day2::main,
        day3::main,
        day4::main,
        day5::main,
        day6::main,
        day7::main,
        day8::main,
    ];
    let assets = Search::ParentsThenKids(3, 3).for_folder("inputs").unwrap();
    let input_path = assets.join(std::format!(
        "day{}{}.txt",
        DAY,
        if IS_TEST { "-test" } else { "" }
    ));
    let input = read_to_string(input_path).unwrap();
    days[DAY - 1](input, PART == 1);
}
