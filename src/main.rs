#![allow(clippy::style)]

use clap::{arg, command, value_parser};

use tracing::error;
// mod lib;
mod aoc2020;
mod aoc2021;

fn main() {
    tracing_subscriber::fmt::init();

    let matches = command!() // requires `cargo` feature
        .arg(
            arg!(-y --year <YEAR>"the year of the puzzle")
            .value_parser(value_parser!(String))
        )
        .arg(
            arg!(-d --day <DAY> "the day of the puzzle")
            .value_parser(value_parser!(u8))
        )
        .arg(
            arg!(-p --part <PART> "the part of the puzzle")
            .value_parser(value_parser!(u8))
        )
        .get_matches();

    let year = matches.get_one::<String>("year")
        .unwrap_or_else(|| panic!("year should be an string")).as_str();

    let day = matches.get_one::<u8>("day")
        .unwrap_or_else(|| panic!("year should be an string"));

    let part = matches.get_one::<u8>("part")
        .unwrap_or_else(|| panic!("year should be an string"));

    match year {
        "2020" =>  aoc2020::run_day(day.to_owned(), part.to_owned()),
        "2021" =>  aoc2021::run_day(day.to_owned(), part.to_owned()),
        _ => error!("year should be 2020 or 2021 not {year}")
    }
}
