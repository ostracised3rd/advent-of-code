use tracing::error;
use advent_of_rust::Day;

mod d01;
mod d02;
mod d03;
mod d04;


pub fn run_day(day: u8, part: u8) {
    match day {
        1 => d01::AoC.run("storage/aoc2023/day01.txt", part),
        2 => d02::AoC.run("storage/aoc2023/day02.txt", part),
        3 => d03::AoC.run("storage/aoc2023/day03.txt", part),
        4 => d04::AoC.run("storage/aoc2023/day04.txt", part),
        _ => error!("day not found {}", day),
    }
}
