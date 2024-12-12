use regex::Regex;
use prse::parse;

pub struct AoC;

impl advent_of_rust::Day for AoC {
    fn p1(&self, _data: String) {
        // let input = data_parser(data);
        // let res = combiner(input);
        // dbg!(res);
    }
    
    fn p2(&self, _data: String) {
        // let actions = data_parser_action(&data);
        // let data = data_parser_indexed(data);
        // let act_rng = action_range(actions);
        // let res = combiner_in_range(act_rng, data);
        // dbg!(res);
    }
}

// fn data_parser_action(raw: &str) -> Vec<(usize, bool)> {
//     let reg = Regex::new(r"do\(\)|don't\(\)").unwrap();
//     reg.find_iter(&raw)
//         .map(|mat: regex::Match<'_>|  (mat.start(), mat.as_str() == "do()"))
//         .collect()
// }

// fn data_parser_indexed(raw: String) -> Vec<(usize, i64, i64)> {
//     let reg = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
//     reg.find_iter(&raw)
//         .map(|mat: regex::Match<'_>|  {
//             let (a, b): (&str, &str) = parse!(mat.as_str(), "mul({},{})");
//             (mat.start(), a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
//         })
//         .collect()
// }

fn data_parser(raw: String) -> Vec<(i64, i64)> {
    let reg = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    reg.find_iter(&raw)
        .map(|mat|  {
            let (a, b): (&str, &str) = parse!(mat.as_str(), "mul({},{})");
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .collect()
}

// fn combiner(data: Vec<(i64, i64)>) -> i64 {
//     data.iter().map(|(a, b)| a * b).sum()
// }

// fn action_range(actions:  Vec<(usize, bool)>) -> Vec<(usize, usize)> {
//     let mut res = Vec::<usize>::new();
//     res.push(0);
//     let mut state = true;
//     for (i, act) in actions {
//         if state != act {
//             res.push(i);
//         }

//         state = act;
//     }

//     if state {
//         res.push(usize::MAX);
//     } 

//     dbg!(&res);

//     if res.len() % 2 != 0 {
//         panic!("what went wrong?")
//     }

//     let mut r = Vec::<(usize, usize)>::new();
//     for i in 1..res.len() {
//         if i % 2 != 0 {
//             r.push((res[i-1], res[i]))
//         }
//     }

//     return r;
// }

// fn combiner_in_range(rg:  Vec<(usize, usize)>, mul: Vec<(usize, i64, i64)>) -> i64 {
//     let mut sum = 0;
//     for (i, a, b) in mul.iter() {
//         for (start, end) in rg.iter() {
//             if i < start {
//                 break;
//             }

//             if i > end {
//                 continue;
//             }

//             sum += a * b;
//         }
//     }

//     sum
// }


mod tests {
    #![allow(dead_code)]
    use super::*;

    #[test]
    fn map_creation() {
        let raw = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let a = data_parser(raw.to_string());
        assert_eq!(vec![(2,4),(5,5),(11,8),(8,5)], a);
    }

    // #[test]
    // fn combiner_test() {
    //     let raw = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    //     let a = data_parser(raw.to_string());
    //     let res = combiner(a);
        
    //     assert_eq!(161, res)
    // }

    // #[test]
    // fn action_parser_test() {
    //     let raw = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    //     let actions = data_parser_action(raw);
    //     assert_eq!(vec![(20,false),(59,true)], actions)
    // }

    // #[test]
    // fn indexed_parser_test() {
    //     let raw = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    //     let actions = data_parser_indexed(raw.to_string());
    //     assert_eq!(vec![(1, 2, 4), (28, 5, 5), (48, 11, 8), (64, 8, 5)], actions)
    // }

    // #[test]
    // fn action_range_test() {
    //     let raw = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    //     let actions = data_parser_action(raw);
    //     let actions = action_range(actions);
    //     assert_eq!(vec![(0, 20),(59, usize::MAX)], actions)
    // }

    // #[test]
    // fn combiner_range_test() {
    //     let raw = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    //     let actions = data_parser_action(raw);
    //     let data = data_parser_indexed(raw.to_string());
    //     let act_rng = action_range(actions);
    //     assert_eq!(48, combiner_in_range(act_rng, data))
    // }
}