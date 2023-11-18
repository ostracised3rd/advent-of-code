use tracing::error;
// mod lib;
mod aoc2020;
mod aoc2021;

fn main() {
    tracing_subscriber::fmt::init();

    let year = "2020";
    let day = "day2";
    let part = advent_of_code::Part::P2;

    match year {
        "2020" =>  aoc2020::run_day(day, part),
        "2021" =>  aoc2021::run_day(day, part),
        _ => error!("year not implemented {}", year)
    }
}
