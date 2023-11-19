pub struct AoC;

impl advent_of_code::Day for AoC {
    fn p1(&self, data: String) {
        let res = p1(&data);

        tracing::info!("{}", res)
    }
    
    fn p2(&self, data: String) {
        let res = p2(&data);

        tracing::info!("{}", res)
    }
}


fn p1(data: &str) -> i32 {
    let res = data.lines()
        .fold((0, 0), |(x, y), s| {
            let (command, val) = s.split_once(' ').unwrap();
            let val: i32 = val.trim().parse().unwrap();
            match command  {
                "down" =>    (x, y+val),
                "forward" => (x+val, y),
                "up" =>      (x, y-val),
                _ => panic!("why am I here?") 
            }
        });

    res.0 * res.1
}


fn p2(data: &str) -> i64 {
    let res = data.lines()
        .fold((0, 0, 0), |(x, y, aim), s| {
            let (command, val) = s.split_once(' ').unwrap();
            let val: i64 = val.trim().parse().unwrap();
            match command  {
                "down" =>    (x, y, aim+val),
                "forward" => (x+val, y+(val*aim), aim),
                "up" =>      (x, y, aim-val),
                _ => panic!("why am I here?")
            }
        });

    res.0 * res.1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_check() {
        let data = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        assert_eq!(150, p1(data));
    }

    #[test]
    fn aimed_position_check() {
        let data = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        assert_eq!(900, p2(data));
    }
}