use tracing::info;
pub struct AoC;

impl advent_of_rust::Day for AoC {
    fn p1(&self, data: String) {
        info!("{data}");
    }
    
    fn p2(&self, data: String) {
        info!("{data}");
    }
}


fn parse_map(raw: &str) {
    let mut numbers = Vec::<(i32, (usize, usize))>::new();
    let mut numbers = Vec::<(i32, (usize, usize))>::new();
    for (j, line) in raw.lines().enumerate() {
        for (i, block) in line.chars().enumerate() {
            if block.is_ascii_digit() {
                numbers.push((block.to_string().parse::<i32>().unwrap(), (i, j)));
                continue;
            }

            if block != '.' {

            }
        }
    }
}



mod tests {
    use super::*;


    #[test]
    fn parse_test() {
        let raw = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let m = vec![
            vec!["4,6,7,.,.,1,1,4,.,."],
            vec![".,.,.,*,.,.,.,.,.,."],
            vec![".,.,3,5,.,.,6,3,3,."],
            vec![".,.,.,.,.,.,#,.,.,."],
            vec!["6,1,7,*,.,.,.,.,.,."],
            vec![".,.,.,.,.,+,.,5,8,."],
            vec![".,.,5,9,2,.,.,.,.,."],
            vec![".,.,.,.,.,.,7,5,5,."],
            vec![".,.,.,$,.,*,.,.,.,."],
            vec![".,6,6,4,.,5,9,8,.,."],
        ];


        let res = vec![467, 35, 633, 617, 592, 755, 664, 598];

        // assert_eq!(res, parse_map(raw))
    }

    #[test]
    fn validity_test() {
    }

    #[test]
    fn minimum_set_test() {
    }
}