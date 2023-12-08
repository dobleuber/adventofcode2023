// ignore dead code warnings
#![allow(dead_code)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod utils;

use day7::day7;

fn main() {
    let result = day7().unwrap();
    println!("{}", result);
}
