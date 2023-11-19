use std::fs;
use tracing::error;

pub fn load_data(filename: &str) -> String {
    fs::read_to_string(filename).unwrap_or_else(|_| panic!("file {} couldn't be read", filename))
}

pub trait Day {
    fn p1(&self, data: String);
    fn p2(&self, data: String);
    fn run(&self, input_file: &str, part: u8) {
        let data = load_data(input_file);
        match part {
            1 => self.p1(data),
            2 => self.p2(data),
            _ => error!("doesn't exists")
        }
    }
}

