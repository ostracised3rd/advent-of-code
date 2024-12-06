use std::collections::HashMap;
use prse::parse;

pub struct AoC;

impl advent_of_rust::Day for AoC {
    fn p1(&self, data: String) {
        let (l1,l2) = data_parser(data);
        let rules = rule_parser(l1);
        let res = validation(l2, &rules);
        dbg!(res);
    }
    
    fn p2(&self, data: String) {
        let (l1,l2) = data_parser(data);
        let rules = rule_parser(l1);
        let res = incorrect_count(l2, &rules);
        dbg!(res);
    }
}

fn rule_parser(data: Vec<(i64, i64)>) -> HashMap<i64, HashMap<i64, bool>> {
    let mut r_map:HashMap<i64, HashMap<i64, bool>> = HashMap::new();
    for (first, last) in data {
        r_map.entry(first)
            .and_modify(|l| {
                l.insert(last, true);
                return 
            })
            .or_insert(HashMap::from([(last,true)]));
    }

    return r_map
}


fn data_parser(raw: String) -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
    let raw_split: Vec<&str> = raw.split("\n\n").collect();

    let raw_rule: Vec<(i64, i64)> = raw_split[0].lines()
        .map(|line| {
            let (a, b): (&str, &str) = parse!(line, "{}|{}");
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        }).collect();

    let print_list: Vec<Vec<i64>> = raw_split[1].lines()
        .map(|line| line.split(",")
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>())
        .collect();

    (raw_rule, print_list)
}


fn validation(list: Vec<Vec<i64>>, rules: &HashMap<i64, HashMap<i64, bool>>) -> i64 {
    list.iter().map(|row| valid_print(row, rules)).sum()
}

fn valid_print(row: &Vec<i64>, rules: &HashMap<i64, HashMap<i64, bool>>) -> i64 {
    for (i, page) in row.iter().enumerate() {
        if rules.contains_key(page) {
            for j in 0..i {
                if rules[page].contains_key(&row[j]) {
                    return 0
                }
            } 
        }
    }

    return row[row.len()/2]
}


fn incorrect_count(list: Vec<Vec<i64>>, rules: &HashMap<i64, HashMap<i64, bool>>) -> i64 {
    list.iter().map(|row| filter_incorrect(row, rules)).sum()
}

fn filter_incorrect(row: &Vec<i64>, rules: &HashMap<i64, HashMap<i64, bool>>) -> i64 {
    for (i, page) in row.iter().enumerate() {
        if rules.contains_key(page) {
            for j in 0..i {
                if rules[page].contains_key(&row[j]) {
                    
                    return correction(i, j, row, rules);
                }
            } 
        }
    }

    return 0
}

fn correction(i: usize, j: usize, row: &Vec<i64>, rules: &HashMap<i64, HashMap<i64, bool>>) -> i64 {
    let corrected = reorder(i, j, row);

    for (i, page) in corrected.iter().enumerate() {
        if rules.contains_key(page) {
            for j in 0..i {
                if rules[page].contains_key(&corrected[j]) {
                    return correction(i, j, &corrected, rules);
                }
            } 
        }
    }

    dbg!(&corrected, &corrected[corrected.len()/2]);
    return corrected[corrected.len()/2]
}

fn reorder(i: usize, j: usize, list: &Vec<i64>) -> Vec<i64> {
    let mut corrected = Vec::<i64>::new();
    for y in 0..list.len() {
        if y == i {
            corrected.push(list[j])
        } else if y == j {
            corrected.push(list[i])
        } else {
            corrected.push(list[y])
        }
    }

    return corrected
}


mod tests {
    use super::*;

    #[test]
    fn map_creation() {
        let raw = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    let (rules,plist) = data_parser(raw.to_string());
        assert_eq!(vec![
            (47,53),
            (97,13),
            (97,61),
            (97,47),
            (75,29),
            (61,13),
            (75,53),
            (29,13),
            (97,29),
            (53,29),
            (61,53),
            (97,53),
            (61,29),
            (47,13),
            (75,47),
            (97,75),
            (47,61),
            (75,61),
            (47,29),
            (75,13),
            (53,13),
        ], rules);
        assert_eq!(vec![
            vec![75,47,61,53,29],
            vec![97,61,53,29,13],
            vec![75,29,13],
            vec![75,97,47,61,53],
            vec![61,13,29],
            vec![97,13,75,29,47],
        ], plist);
    }

    #[test]
    fn validation_test() {
        let raw = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let (l1,l2) = data_parser(raw.to_string());
        let rules = rule_parser(l1);
        let res = validation(l2, &rules);
        assert_eq!(res, 143)
    }

    #[test]
    fn incorrect_count_test() {
        let raw = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let (l1,l2) = data_parser(raw.to_string());
        let rules = rule_parser(l1);
        let res = incorrect_count(l2, &rules);
        assert_eq!(res, 123)
    }
}