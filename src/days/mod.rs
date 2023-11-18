use tracing::error;
use crate::helpers;

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


pub enum Part {
    P1,
    P2,
}

trait Day {
    fn p1(&self, data: String);
    fn p2(&self, data: String);
    fn run(&self, input_file: &str, part: Part) {
        let data = helpers::load_data(input_file);
        match part {
            Part::P1 => self.p1(data),
            Part::P2 => self.p2(data),
        }
    }
}

pub fn run_day(day: &str, part: Part) {
    match day {
        "day1" => d1::AoC.run("storage/day1.txt", part),
        "day2" => d2::Day.run("storage/day2.txt", part),
        _ => error!("day not found {}", day),
    }
}
