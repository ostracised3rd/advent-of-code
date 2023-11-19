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
        "day1" => d01::AoC.run("storage/day1.txt", part),
        "day2" => d02::AoC.run("storage/day2.txt", part),
        "day3" => d03::AoC.run("storage/day3.txt", part),
        "day4" => d04::AoC.run("storage/day4.txt", part),
        _ => error!("day not found {}", day),
    }
}
