use std::collections::HashMap;

pub struct AoC;

impl advent_of_rust::Day for AoC {
    fn p1(&self, data: String) {
        let res: u32 = data.lines()
            .map(|l| num_ex_str(l))
            .sum();

        dbg!(res);
       
    }
    
    fn p2(&self, data: String) {

        let mut num: HashMap<String, u32> =  HashMap::new();
        num.insert("one".into(), 1);
        num.insert("two".into(), 2);
        num.insert("three".into(), 3);
        num.insert("four".into(), 4);
        num.insert("five".into(), 5);
        num.insert("six".into(), 6);
        num.insert("seven".into(), 7);
        num.insert("eight".into(), 8);
        num.insert("nine".into(), 9);

        let res: u32 = data.lines()
            .map(|l| index_ex_num_str(l, &num))
            .sum();
        
        dbg!(res);
    }
}

fn index_ex_num_str(line: &str, num_str: &HashMap<String, u32>) -> u32 {

    let mut first: (usize, u32) = (usize::MAX, 0); 
    let mut last: (usize, u32) = (usize::MIN, 0);
    
    for (k, v) in num_str.iter() {
        let mut matched: Vec<usize> = line.match_indices(k).map(|x| x.0).collect();
        matched.append(&mut line.match_indices(&v.to_string()).map(|x| x.0).collect::<Vec<usize>>());

        for i in matched {
            if first.0 >= i {
                first.0 = i;
                first.1 = *v;
            }

            if last.0 <= i {
                last.0 = i;
                last.1 = *v;
            }
        }
    }

    if first.1 == 0 || last.1 == 0 {
        dbg!(first, last);
    }

    (first.1 * 10) + last.1
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

    #[test]
    fn num_from_str() {
        let mut num: HashMap<String, u32> =  HashMap::new();
        num.insert("one".into(), 1);
        num.insert("two".into(), 2);
        num.insert("three".into(), 3);
        num.insert("four".into(), 4);
        num.insert("five".into(), 5);
        num.insert("six".into(), 6);
        num.insert("seven".into(), 7);
        num.insert("eight".into(), 8);
        num.insert("nine".into(), 9);

        assert_eq!(29, index_ex_num_str("two1nine".into(), &num));
        assert_eq!(83, index_ex_num_str("eightwothree".into(), &num));
        assert_eq!(13, index_ex_num_str("abcone2threexyz".into(), &num));
        assert_eq!(24, index_ex_num_str("xtwone3four".into(), &num));
        assert_eq!(42, index_ex_num_str("4nineeightseven2".into(), &num));
        assert_eq!(14, index_ex_num_str("zoneight234".into(), &num));
        assert_eq!(76, index_ex_num_str("7pqrstsixteen".into(), &num));

        assert_eq!(12, index_ex_num_str("1abc2", &num));
        assert_eq!(38, index_ex_num_str("pqr3stu8vwx", &num));
        assert_eq!(15, index_ex_num_str("a1b2c3d4e5f", &num));
        assert_eq!(77, index_ex_num_str("treb7uchet", &num));

        assert_eq!(82, index_ex_num_str("8262sixthreepxvgpqf4two", &num));
        
    }
}