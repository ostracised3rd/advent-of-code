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
mod d09;
mod d16;
mod d17;


pub fn run_day(day: &str, part: u8) {
    match day {
        "day1" => d01::AoC.run("storage/aoc2020/day1.txt", part),
        "day2" => d02::AoC.run("storage/aoc2020/day2.txt", part),
        "day3" => d03::AoC.run("storage/aoc2020/day3.txt", part),
        "day4" => d04::AoC.run("storage/aoc2020/day4.txt", part),
        "day5" => d05::AoC.run("storage/aoc2020/day5.txt", part),
        "day6" => d06::AoC.run("storage/aoc2020/day6.txt", part),
        "day7" => d07::AoC.run("storage/aoc2020/day7.txt", part),
        "day8" => d08::AoC.run("storage/aoc2020/day8.txt", part),
        "day9" => d09::AoC.run("storage/aoc2020/day9.txt", part),
        "day16" => d16::AoC.run("storage/aoc2020/day16.txt", part),
        "day17" => d17::AoC.run("storage/aoc2020/day17.txt", part),
        _ => error!("day not found {}", day),
    }
}
