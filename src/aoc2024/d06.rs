use std::collections::HashMap;

pub struct AoC;

impl advent_of_rust::Day for AoC {
    fn p1(&self, data: String) {
        let (map_list, actor) = data_parser(data);
        let res = walking(actor, map_list);
        dbg!(res);
    }
    
    fn p2(&self, _data: String) {
        // let (l1, l2) = data_parser(data);
        // let m2 = into_map(l2);
        // let res = similarity(l1, m2);
        // dbg!(res);
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point {
            x: x as usize,
            y: y as usize,
        }
    } 
}

#[derive(Clone, Copy, Debug)]
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


#[derive(Debug)]
struct Actor {
    loc: Point,
    dir: Dir,
    dir_index: usize,
    walk: HashMap<Point, i64>,
}

impl Actor {
    fn new(loc: Point) -> Self {
        Actor {
            loc: loc.clone(),
            walk: HashMap::from([(loc, 1)]),
            dir: UP,
            dir_index:  0,
        }
    }

    fn set_step(&mut self, next: Point) {
        self.loc = next;
        self.walk.entry(next).and_modify(|c| *c += 1).or_insert(1);
    }

    fn turn(&mut self) {
        self.dir_index = (self.dir_index + 1) % 4;
        self.dir = Directions[self.dir_index];
    }

    fn next(&self) -> (i64, i64) {
        (self.loc.x as i64 + self.dir.x, self.loc.y as i64 + self.dir.y)
    }

    fn step_count(&self) -> i64 {
        self.walk.len() as i64
    }
}


fn data_parser(raw: String) -> (Matrix, Actor) {
    let mut map_list: Matrix = Vec::new();
    let mut start = Point{x:0, y:0};

    for (y, line) in raw.lines().enumerate() {
        map_list.push(Vec::new());
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    map_list[y].push(0);
                },
                '#' => {
                    map_list[y].push(1);
                },
                '^' => {
                    map_list[y].push(0);
                    start =  Point{x, y};
                },
                _ => {
                    map_list[y].push(0);
                }
            }
        }
    }

    (map_list, Actor::new(start))
}

fn walking(mut actor: Actor, map: Matrix) -> i64 {
    loop {
        if actor.loc.x == 0 || 
            actor.loc.x == map[0].len() - 1 ||
            actor.loc.y == 0 || 
            actor.loc.y == map.len() - 1 {
            break;
        }

        let (x, y) = actor.next();
        let p = Point::new(x, y);

        if map[p.y][p.x] == 1 {
            actor.turn();
        } else {
            actor.set_step(p);
        }
    }

    return actor.step_count()
}

mod tests {
    use super::*;

    #[test]
    fn walking_test() {
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

        let (map_list, actor) = data_parser(raw.to_string());
        let count = walking(actor, map_list);
        assert_eq!(41, count);
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