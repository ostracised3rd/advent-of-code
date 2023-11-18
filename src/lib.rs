use std::fs;


pub fn load_data(filename: &str) -> String {
    fs::read_to_string(filename).expect(&format!("file {} couldn't be read", filename))
}

pub enum Part {
    P1,
    P2,
}

pub trait Day {
    fn p1(&self, data: String);
    fn p2(&self, data: String);
    fn run(&self, input_file: &str, part: Part) {
        let data = load_data(input_file);
        match part {
            Part::P1 => self.p1(data),
            Part::P2 => self.p2(data),
        }
    }
}

