pub struct AoC;

impl advent_of_rust::Day for AoC {
    fn p1(&self, data: String) {
        let ll = data_parser(data);
        let count = safe_count(ll, is_safe);
        dbg!(count);
    }
    
    fn p2(&self, data: String) {
        let ll = data_parser(data);
        let count = safe_count(ll, is_dampener_safe);
        dbg!(count);
    }
}



fn data_parser(raw: String) -> Vec<Vec<i64>> {
    raw.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|a| a.parse::<i64>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i64>>>()
}

type SafetyCheck = fn(&Vec<i64>, i64) -> i64;


fn is_safe(row: &Vec<i64>, _t: i64) -> i64 {
    let dif = row[1] - row[0];
    if dif == 0 {
        return 0
    }
    let dir =  (dif) / (dif).abs();
    let mut s = row[0];
    for i in 1..row.len() {
        if dir * row[i] < 1 + s || dir * row[i] > 3 + s {
            return 0
        }

        s = row[i];
    }

    return 1
}

fn rm_fault(row: &Vec<i64>, index: usize) -> Vec<i64> {
    let mut r: Vec<i64> = Vec::new();
        for j in 0..row.len() {
            if j == index {
                continue;
            }

            r.push(row[j]);
        } 

    return r
}

fn is_dampener_safe(row: &Vec<i64>, t: i64) -> i64 {
    dbg!(&row);
    let dif = row[1] - row[0];
    if dif == 0 {
        if t == 0 {
            let r1 = rm_fault(row, 0);
            let r2 = rm_fault(row, 1);

            let res1 = is_dampener_safe(&r1, 1);
            let res2 = is_dampener_safe(&r2, 1);

            return res1.max(res2)
        }
        return 0
    }
    let dir =  (dif) / (dif).abs();
    let mut s = row[0];
    for i in 1..row.len() {
        let min = (dir * 1) + s;
        let max = (dir * 3) + s;
        if dir * row[i] < dir * min || dir * row[i] > dir * max {
            if t == 0 {
                let r1 = rm_fault(row, i);
                let r2 = rm_fault(row, i-1);

                let res1 = is_dampener_safe(&r1, 1);
                let res2 = is_dampener_safe(&r2, 1);

                return res1.max(res2)
            }
            return 0
        }

        s = row[i];
    }

    dbg!(1);
    return 1
}

fn safe_count(data: Vec<Vec<i64>>, func: SafetyCheck) -> i64 {
    data.iter()
        .map(|row| func(row, 0))
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
        let count = safe_count(ll, is_safe);
        dbg!(&count);
        assert_eq!(count, 2)
    }

    #[test]
    fn safe_dampener_count_test() {
        let raw = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        let ll = data_parser(raw.to_string());
        let count = safe_count(ll, is_dampener_safe);
        dbg!(&count);
        assert_eq!(count, 4)
    }
}



// fn is_damp_safe(row: &Vec<i64>) -> i64 {
//     let mut dif = Vec::<i64>::new();
//     let mut i = 0;
//     loop {
//         if i + 1 > row.len() { break; }

//         dif.push(row[i+1] - row[i]);
//         i += 1;
//     }

//     for c in dif {
        
//     }

//     return 1
// }


// fn is_damp_safe(row: &Vec<i64>) -> i64 {
//     let mut damped = false;

//     let mut dif = Vec::<i64>::new();
//     let mut dir = Vec::<i64>::new();
//     for i in 1..row.len() {
//         let step = row[i] - row[i-1];

//         if step != 0 {
//             dir.push(step / step.abs())
//         }
//         dif.push(step);
//     }






//     if row.len() < 3 {
//         let a = (row[1] - row[0]).abs();
//         if a <= 3 || a >= 1 {
//             return 1
//         } else {
//             return 0
//         }
//     }



//     for i in 2..row.len() {
//         let a = row[i-1] - row[i-2];
    
//     }


//     return 1
// }

// fn is_dampener_safe(row: &Vec<i64>) -> i64 {
//     let mut dampened = false;
//     let mut reverse = false;
//     let mut jump = 0;
//     let mut dif = row[1] - row[0];
//     if dif == 0 && !dampened {
//         dif = row[2] - row[0];
//         dampened = true;
//     }

//     if dif == 0 {
//         return 0
//     }
//     let mut dir =  (dif) / (dif).abs();
//     let mut s = row[0];
//     let mut i = 0;
//     loop {
//         i += 1;
//         if i >= row.len() {
//             break;
//         }

//         if i == jump {
//             continue;
//         }

//         let min = (dir * 1) + s;
//         let max = (dir * 3) + s;
//         if dir * row[i] < dir * min || dir * row[i] > dir * max {
//             jump = i;
//             if !dampened {
//                 dampened = true;
//                 continue;
//             }

//             if !reverse {
//                 reverse = true;
//                 i = 0;
//                 dir *= -1;
//                 continue;
//             }

//             return 0
//         }

//         s = row[i];
//     }

//     return 1
// }

