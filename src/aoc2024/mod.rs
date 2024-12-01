use tracing::error;
use advent_of_rust::Day;

mod d01;


pub fn run_day(day: u8, part: u8) {
    match day {
        1 => d01::AoC.run("storage/aoc2024/day01.txt", part),
        _ => error!("day not found {}", day),
    }
}
