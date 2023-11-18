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


pub fn run_day(day: &str, part: advent_of_code::Part) {
    match day {
        // "day1" => d1::AoC.run("storage/day1.txt", part),
        // "day2" => d2::AoC.run("storage/day2.txt", part),
        _ => error!("day not found {}", day),
    }
}