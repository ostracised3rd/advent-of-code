use tracing::error;
use advent_of_code::Day;

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;
mod d16;
mod d17;


pub fn run_day(day: &str, part: advent_of_code::Part) {
    match day {
        "day1" => d1::AoC.run("storage/day1.txt", part),
        "day2" => d2::AoC.run("storage/day2.txt", part),
        _ => error!("day not found {}", day),
    }
}
