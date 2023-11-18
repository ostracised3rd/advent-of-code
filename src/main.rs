
mod helpers;
mod days;

fn main() {
    tracing_subscriber::fmt::init();

    let day = "day2";
    let part = days::Part::P2;

    days::run_day(day, part);
}
