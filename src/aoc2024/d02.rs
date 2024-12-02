pub struct AoC;

impl advent_of_rust::Day for AoC {
    fn p1(&self, data: String) {
        let ll = data_parser(data);
        let count = safe_count(ll);
        dbg!(count);
    }
    
    fn p2(&self, _data: String) {
        // let (l1, l2) = data_parser(data);
        // let m2 = into_map(l2);
        // let res = similarity(l1, m2);

        
        
        // dbg!(res);
    }
}



fn data_parser(raw: String) -> Vec<Vec<i64>> {
    raw.lines()
        .map(|line| {
            line.split_whitespace()
                .into_iter()
                .map(|a| a.parse::<i64>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i64>>>()
}


fn is_safe(row: &Vec<i64>) -> i64 {
    println!("next row!");

    let dif = row[1] - row[0];
    if dif == 0 {
        return 0
    }
    let dir =  (dif) / (dif).abs();
    dbg!(&dir);
    let mut s = row[0];
    dbg!(&s);
    for i in 1..row.len() {
        dbg!(&row[i]);
        let min = (dir * 1) + s;
        dbg!(&min);
        let max = (dir * 3) + s;
        dbg!(&max);
        if dir * row[i] < dir * min || dir * row[i] > dir * max {
            return 0
        }

        s = row[i];
        dbg!("s", &s);
    }

    1
}

fn safe_count(data: Vec<Vec<i64>>) -> i64 {
    data.iter()
        .map(|row| is_safe(row))
        // .collect::<Vec<i64>>()
        // .iter()
        .sum()
}


mod tests {
    use super::*;

    #[test]
    fn map_creation() {
        let raw = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    let ll = data_parser(raw.to_string());
        assert_eq!(vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ], ll);
    }

    #[test]
    fn safe_count_test() {
        let raw = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        let ll = data_parser(raw.to_string());
        let count = safe_count(ll);
        dbg!(&count);
        assert_eq!(count, 2)
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
        let _ll = data_parser(raw.to_string());
        // let m2 = into_map(l2);
        // let res = similarity(l1, m2);
        // assert_eq!(res, 31)
    }
}