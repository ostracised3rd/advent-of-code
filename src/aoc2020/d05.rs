pub struct AoC;

impl advent_of_code::Day for AoC {
    fn p1(&self, data: String) {
        let sits = data.lines().collect();
        let res = highest_sit_id(sits);
        tracing::info!("{}", res)
    }
    
    fn p2(&self, data: String) {
        let sits = data.lines().collect();
        let res = find_sit(sits);
        tracing::info!("{}", res)
    }
}



fn bsp(text: &str, lower: f32, upper: f32, rule: char)  -> u32 {
    text
        .chars()
        .fold((lower, upper), |(l, h), c| {
            let mid: f32 = l + ((h - l) / 2.);
            if c == rule { (mid.ceil(), h) } else { (l, mid.ceil()) }
        }).0 as u32
}


fn row_num(raw: &str) -> u32 {
    bsp(raw, 0., 127., 'B')
}


fn column_num(raw: &str) -> u32 {
    bsp(raw, 0., 7., 'R')
}


fn sit_num(raw: &str) -> u32 {
    let row = row_num(&raw[..7]);
    let column = column_num(&raw[7..]);
    row * 8 + column
}


fn highest_sit_id(sits: Vec<&str>) -> u32 {
    sits.iter().map(|x| sit_num(x)).max().unwrap()
}


fn find_sit(sits: Vec<&str>) -> i32 {

    let mut list = sits.iter()
        .map(|x| sit_num(x) as i32)
        .collect::<Vec<i32>>();
    
    list.sort();

    let lower = list.iter().min().unwrap_or(&0);
    let upper = list.iter().max().unwrap_or(&0);
    let real = ((upper + lower) * (upper - lower + 1)) / 2;

    real - list.iter().sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_row_check() {
        assert_eq!(44, row_num(&"FBFBBFFRLR"[..7]));
        assert_eq!(70, row_num(&"BFFFBBFRRR"[..7]));
        assert_eq!(14, row_num(&"FFFBBBFRRR"[..7]));
        assert_eq!(102, row_num(&"BBFFBBFRLL"[..7]));
    }

    #[test]
    fn p1_column_check() {
        assert_eq!(5, column_num(&"FBFBBFFRLR"[7..]));
        assert_eq!(7, column_num(&"BFFFBBFRRR"[7..]));
        assert_eq!(7, column_num(&"FFFBBBFRRR"[7..]));
        assert_eq!(4, column_num(&"BBFFBBFRLL"[7..]));
    }

    #[test]
    fn p1_sit_check() {
        assert_eq!(119, sit_num(&"FFFBBBFRRR"));
        assert_eq!(820, sit_num(&"BBFFBBFRLL"));
        assert_eq!(567, sit_num(&"BFFFBBFRRR"));
        assert_eq!(357, sit_num(&"FBFBBFFRLR"));
    }

    #[test]
    fn max_check() {
        let sits = vec![
            "FFFBBBFRRR",
            "BBFFBBFRLL",
            "BFFFBBFRRR",
            "FBFBBFFRLR",
        ];

        assert_eq!(820, highest_sit_id(sits));
    }
}