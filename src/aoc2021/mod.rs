use tracing::error;
use advent_of_code::Day;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;


pub fn run_day(day: &str, part: u8) {
    match day {
        "day1" => d01::AoC.run("storage/aoc2021/day1.txt", part),
        "day2" => d02::AoC.run("storage/aoc2021/day2.txt", part),
        "day3" => d03::AoC.run("storage/aoc2021/day3.txt", part),
        "day4" => d04::AoC.run("storage/aoc2021/day4.txt", part),
        "day5" => d05::AoC.run("storage/aoc2021/day5.txt", part),
        "day6" => d06::AoC.run("storage/aoc2021/day6.txt", part),
        "day7" => d07::AoC.run("storage/aoc2021/day7.txt", part),
        "day8" => d08::AoC.run("storage/aoc2021/day8.txt", part),
        _ => error!("day not found {}", day),
    }
}