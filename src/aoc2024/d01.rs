use std::collections::HashMap;

pub struct AoC;

impl advent_of_rust::Day for AoC {
    fn p1(&self, data: String) {
        let (l1, l2) = data_parser(data);
        let res = difference(l1, l2);
        dbg!(res);
    }
    
    fn p2(&self, data: String) {
        let (l1, l2) = data_parser(data);
        let m2 = into_map(l2);
        let res = similarity(l1, m2);

        
        
        dbg!(res);
    }
}

fn data_parser(raw: String) -> (Vec<i64>, Vec<i64>) {
    let mut fl: Vec<i64> = Vec::new();
    let mut sl: Vec<i64> = Vec::new();
    for line in raw.lines() {
        let mut a = line.split_whitespace();
        fl.push(a.next().unwrap().parse::<i64>().unwrap());
        sl.push(a.next().unwrap().parse::<i64>().unwrap());
    }

    (fl, sl)
}

fn into_map(l: Vec<i64>) -> HashMap<i64, i64> {
    let mut m: HashMap<i64, i64> = HashMap::new();

    for i in l{
        m.entry(i).and_modify(|counter| *counter += 1).or_insert(1);
    }

    m
}

fn difference(mut l1: Vec<i64>, mut l2: Vec<i64>) -> i64 {
    l1.sort();
    l2.sort();

    let mut res: i64 = 0; 
    for (i, _) in l1.iter().enumerate() {
        let a = l2[i] - l1[i];
        
        res += if a >= 0 {
            a
        } else {
            -a
        }
    }

    res
}

fn similarity(l1: Vec<i64>, m2: HashMap<i64, i64>) -> i64 {
    let mut res = 0;
    for row in l1 {
        let mult = m2.get(&row).unwrap_or(&0);
        res += (row * *mult);
    }

    res
}

mod tests {
    use super::*;

    #[test]
    fn map_creation() {
        let raw = "3   4
4   3
2   5
1   3
3   9
3   3
";

    let (l1,l2) = data_parser(raw.to_string());
        assert_eq!(vec![3,4,2,1,3,3], l1);
        assert_eq!(vec![4,3,5,3,9,3], l2);
    }

    #[test]
    fn difference_test() {
        let raw = "3   4
4   3
2   5
1   3
3   9
3   3
";
        let (l1,l2) = data_parser(raw.to_string());
        let res = difference(l1, l2);
        assert_eq!(res, 11)
    }

    #[test]
    fn similarity_test() {
        let raw = "3   4
4   3
2   5
1   3
3   9
3   3
";
        let (l1,l2) = data_parser(raw.to_string());
        let m2 = into_map(l2);
        let res = similarity(l1, m2);
        assert_eq!(res, 31)
    }
}