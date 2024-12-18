extern crate find_folder;
extern crate regex;

use find_folder::Search;
use std::fs::read_to_string;

const INFO: (usize, usize, usize) = (18, 2, 0);
const DAY: usize = INFO.0;
const PART: usize = INFO.1;
const IS_TEST: bool = INFO.2 == 1;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
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
        day9::main,
        day10::main,
        day11::main,
        day12::main,
        day13::main,
        day14::main,
        day15::main,
        day16::main,
        day17::main,
        day18::main,
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
