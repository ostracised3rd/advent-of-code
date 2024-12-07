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


struct Point {
    x: usize,
    y: usize,
}

struct Dir {
    x: i64,
    y: i64,
}

type Matrix = Vec<Vec<i64>>;

const UP: Dir = Dir{x:0, y:-1};
const RIGHT: Dir = Dir{x:1, y:0};
const DOWN: Dir = Dir{x:0, y:1};
const LEFT: Dir = Dir{x:-1, y:0};

const Directions: [Dir; 4] = [
    UP,
    RIGHT,
    DOWN,
    LEFT,
];



struct Actor {
    location: Point,
    direction: Dir,
}

impl Actor {
    fn step(&mut self) {
        
    }
}


fn data_parser(raw: String) -> (Matrix, Matrix, Actor) {
    let mut map_list: Matrix = Vec::new();
    let mut walk_list: Matrix = Vec::new();
    let mut actor: Actor = Actor{location: Point{x:0, y:0}, direction: UP}; 

    for (y, line) in raw.lines().enumerate() {
        map_list.push(Vec::new());
        walk_list.push(Vec::new());
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    map_list[y].push(0);
                    walk_list[y].push(0);
                },
                '#' => {
                    map_list[y].push(1);
                    walk_list[y].push(0);
                },
                '^' => {
                    map_list[y].push(0);
                    walk_list[y].push(1);
                    actor.location = Point{x, y};
                },
                _ => {
                    map_list[y].push(0);
                    walk_list[y].push(0);
                }
            }
        }
    }

    (map_list, walk_list, actor)
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
        // assert_eq!(vec![3,4,2,1,3,3], l1);
        // assert_eq!(vec![4,3,5,3,9,3], l2);
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