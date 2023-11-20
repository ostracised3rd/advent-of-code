use tracing::error;
use advent_of_rust::Day;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;


pub fn run_day(day: u8, part: u8) {
    match day {
        1 => d01::AoC.run("storage/aoc2021/day1.txt", part),
        2 => d02::AoC.run("storage/aoc2021/day2.txt", part),
        3 => d03::AoC.run("storage/aoc2021/day3.txt", part),
        4 => d04::AoC.run("storage/aoc2021/day4.txt", part),
        5 => d05::AoC.run("storage/aoc2021/day5.txt", part),
        6 => d06::AoC.run("storage/aoc2021/day6.txt", part),
        7 => d07::AoC.run("storage/aoc2021/day7.txt", part),
        8 => d08::AoC.run("storage/aoc2021/day8.txt", part),
        _ => error!("day should be between 1 and 25 not {}", day),
    }
}