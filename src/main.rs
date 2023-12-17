// ignore dead code warnings
#![allow(dead_code)]

mod day1;
mod day10;
mod day11;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

use day14::day14 as proccess;

fn main() {
    let result = proccess().unwrap();
    println!("{}", result);
}
