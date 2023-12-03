pub struct AoC;

impl advent_of_rust::Day for AoC {
    fn p1(&self, data: String) {
        let res: u32 = data.lines()
            .map(|l| num_ex_str(l))
            .sum();

        dbg!(res);
       
    }
    
    fn p2(&self, data: String) {
        // let _inputs = data
        //     .lines()
        //     .map(|x| x.trim().parse().unwrap())
        //     .collect();
    
        // let res3 = p2_expense_report(inputs);
        // println!("{}", res3);
    }
}

fn num_ex_str(line: &str) -> u32 {
    let mut num = line.chars()
        .filter(|c| !c.is_alphabetic())
        .map(|x| x.to_string().parse().unwrap())
        .collect::<Vec<u32>>();

    num[0] * 10 + num.pop().unwrap()
}


mod tests {
    use super::*;

    #[test]
    fn map_creation() {
        assert_eq!(12, num_ex_str("1abc2"));
        assert_eq!(38, num_ex_str("pqr3stu8vwx"));
        assert_eq!(15, num_ex_str("a1b2c3d4e5f"));
        assert_eq!(77, num_ex_str("treb7uchet"));
    }
}