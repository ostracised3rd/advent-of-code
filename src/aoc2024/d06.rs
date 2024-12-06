use std::collections::HashMap;

pub struct AoC;

impl advent_of_rust::Day for AoC {
    fn p1(&self, _data: String) {
        // let (l1, l2) = data_parser(data);
        // let res = difference(l1, l2);
        // dbg!(res);
    }
    
    fn p2(&self, _data: String) {
        // let (l1, l2) = data_parser(data);
        // let m2 = into_map(l2);
        // let res = similarity(l1, m2);
        // dbg!(res);
    }
}

fn data_parser(raw: String) -> (Vec<Vec<i64>>, Vec<Vec<i64>>, (usize, usize)) {
    let mut map_list: Vec<Vec<i64>> = Vec::new();
    let mut walk_list: Vec<Vec<i64>> = Vec::new();
    let mut start: (usize, usize) = (0, 0); 

    for (i, line) in raw.lines().enumerate() {
        map_list.push(Vec::new());
        walk_list.push(Vec::new());
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    map_list[i].push(0);
                    walk_list[i].push(0);
                },
                '#' => {
                    map_list[i].push(1);
                    walk_list[i].push(0);
                },
                '^' => {
                    map_list[i].push(0);
                    walk_list[i].push(1);
                    start = (i, j);
                },
                _ => {
                    map_list[i].push(0);
                    walk_list[i].push(0);
                }
            }
        }
    }

    (map_list, walk_list, start)
}

mod tests {
    use super::*;

    #[test]
    fn map_creation() {
        let raw = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let (map_list, walk_list, start) = data_parser(raw.to_string());
        assert_eq!(vec![3,4,2,1,3,3], l1);
        assert_eq!(vec![4,3,5,3,9,3], l2);
    }

//     #[test]
//     fn difference_test() {
//         let raw = "3   4
// 4   3
// 2   5
// 1   3
// 3   9
// 3   3
// ";
//         let (l1,l2) = data_parser(raw.to_string());
//         let res = difference(l1, l2);
//         assert_eq!(res, 11)
//     }

//     #[test]
//     fn similarity_test() {
//         let raw = "3   4
// 4   3
// 2   5
// 1   3
// 3   9
// 3   3
// ";
//         let (l1,l2) = data_parser(raw.to_string());
//         let m2 = into_map(l2);
//         let res = similarity(l1, m2);
//         assert_eq!(res, 31)
//     }
}