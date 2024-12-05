use std::collections::{HashMap, VecDeque};

use tracing::info;
pub struct AoC;

impl advent_of_rust::Day for AoC {
    fn p1(&self, data: String) {
        let (num_coords, sym_coords) = parse_map(&data);
        let num = adj_numbers(num_coords, sym_coords);
        let res: i64 = num.iter().sum();
        info!("{res}");
    }
    
    fn p2(&self, data: String) {
        let (num_coords, sym_coords) = parse_map_to_gear(&data);
        let num = gear_numbers(num_coords, sym_coords);
        let res: i64 = num.iter().sum();
        info!("{res}");
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
struct Coords {
    x: i64,
    y: i64,
}


fn parse_map_to_gear(raw: &str) -> (HashMap<Coords, char>, HashMap<Coords, char>){
    let mut number_coords = HashMap::<Coords, char>::new();
    let mut symbol_coords = HashMap::<Coords, char>::new();

    for (y, line) in raw.lines().enumerate() {
        for (x , block) in line.chars().enumerate() {
            if block.is_ascii_digit() {
                number_coords.insert(Coords{x: x as i64,y: y as i64}, block);
                continue;
            }

            if block == '*' {
                symbol_coords.insert(Coords{x: x as i64,y: y as i64}, block);
            }
        }
    }

    return (number_coords, symbol_coords)
}


fn parse_map(raw: &str) -> (HashMap<Coords, char>, HashMap<Coords, char>){
    let mut number_coords = HashMap::<Coords, char>::new();
    let mut symbol_coords = HashMap::<Coords, char>::new();

    for (y, line) in raw.lines().enumerate() {
        for (x , block) in line.chars().enumerate() {
            if block.is_ascii_digit() {
                number_coords.insert(Coords{x: x as i64,y: y as i64}, block);
                continue;
            }

            if block != '.' {
                symbol_coords.insert(Coords{x: x as i64,y: y as i64}, block);
            }
        }
    }

    return (number_coords, symbol_coords)
}

fn create_num(num_map: &HashMap<Coords, char>, coords: &Coords) -> (HashMap<Coords, bool>, i64) {
    let mut left = Coords{x: coords.x - 1, y: coords.y};
    let mut right = Coords{x: coords.x + 1, y: coords.y};

    let mut num = VecDeque::from([*num_map.get(coords).unwrap()]);
    let mut used: HashMap<Coords, bool> = HashMap::from([(*coords, true)]);

    // println!("{:?}, {:?}, {:?}, {:?}", left, right, num, used);

    while num_map.contains_key(&left) {
        num.push_front(*num_map.get(&left).unwrap());
        used.insert(left, true);
        left.x -= 1;
    }

    while num_map.contains_key(&right) {
        num.push_back(*num_map.get(&right).unwrap());
        used.insert(right, true);
        right.x += 1;
    }

    let num: String = num.iter().collect();

    return (used, num.parse().unwrap())
}

fn adj_numbers(num: HashMap<Coords, char>, sym: HashMap<Coords, char>) -> Vec<i64> {
    let mut numbers = Vec::<i64>::new();
    let mut used = HashMap::<Coords, bool>::new();

    for (k, _) in sym.iter() {
        let coords = [
            Coords{x: k.x-1, y: k.y-1},
            Coords{x: k.x-1, y: k.y},
            Coords{x: k.x-1, y: k.y+1},
            Coords{x: k.x, y: k.y-1},
            Coords{x: k.x, y: k.y+1},
            Coords{x: k.x+1, y: k.y-1},
            Coords{x: k.x+1, y: k.y},
            Coords{x: k.x+1, y: k.y+1},
        ];

        for c in coords.iter() {
            if num.contains_key(c) && !used.contains_key(c) {
                let (u, i) = create_num(&num, c);
                used.extend(u.iter());
                numbers.push(i);
            }
        }
    }

    numbers
}

fn gear_numbers(num: HashMap<Coords, char>, sym: HashMap<Coords, char>) -> Vec<i64> {
    let mut numbers = Vec::<i64>::new();
    

    for (k, _) in sym.iter() {
        let mut used = HashMap::<Coords, bool>::new();
        let mut adj = Vec::<i64>::new(); 
        let coords = [
            Coords{x: k.x-1, y: k.y-1},
            Coords{x: k.x-1, y: k.y},
            Coords{x: k.x-1, y: k.y+1},
            Coords{x: k.x, y: k.y-1},
            Coords{x: k.x, y: k.y+1},
            Coords{x: k.x+1, y: k.y-1},
            Coords{x: k.x+1, y: k.y},
            Coords{x: k.x+1, y: k.y+1},
        ];

        for c in coords.iter() {
            if num.contains_key(c) && !used.contains_key(c) {
                let (u, i) = create_num(&num, c);
                used.extend(u.iter());
                adj.push(i);
            }
        }

        if adj.len() == 2 {
            numbers.push(adj[0] * adj[1])
        }
    }

    numbers
}



mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let raw = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let (num_coords, sym_coords) = parse_map(raw);

        println!("{:?}", num_coords);
        println!("{:?}", sym_coords);

        let num_test = HashMap::from([(Coords { x: 2, y: 6 }, '5'), (Coords { x: 5, y: 0 }, '1'), (Coords { x: 4, y: 6 }, '2'), (Coords { x: 6, y: 9 }, '9'), (Coords { x: 1, y: 4 }, '1'), (Coords { x: 7, y: 7 }, '5'), (Coords { x: 2, y: 4 }, '7'), (Coords { x: 6, y: 0 }, '1'), (Coords { x: 3, y: 2 }, '5'), (Coords { x: 7, y: 5 }, '5'), (Coords { x: 6, y: 2 }, '6'), (Coords { x: 1, y: 0 }, '6'), (Coords { x: 3, y: 6 }, '9'), (Coords { x: 8, y: 5 }, '8'), (Coords { x: 1, y: 9 }, '6'), (Coords { x: 6, y: 7 }, '7'), (Coords { x: 3, y: 9 }, '4'), (Coords { x: 2, y: 2 }, '3'), (Coords { x: 8, y: 2 }, '3'), (Coords { x: 7, y: 9 }, '8'), (Coords { x: 2, y: 9 }, '6'), (Coords { x: 5, y: 9 }, '5'), (Coords { x: 0, y: 0 }, '4'), (Coords { x: 0, y: 4 }, '6'), (Coords { x: 2, y: 0 }, '7'), (Coords { x: 7, y: 0 }, '4'), (Coords { x: 8, y: 7 }, '5'), (Coords { x: 7, y: 2 }, '3')]);
        let sym_test = HashMap::from([(Coords { x: 6, y: 3 }, '#'), (Coords { x: 3, y: 4 }, '*'), (Coords { x: 5, y: 5 }, '+'), (Coords { x: 5, y: 8 }, '*'), (Coords { x: 3, y: 1 }, '*'), (Coords { x: 3, y: 8 }, '$')]);

        assert_eq!(num_test, num_coords);
        assert_eq!(sym_test, sym_coords);
    }

    #[test]
    fn adj_numbers_test() {
        let raw = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let (num_coords, sym_coords) = parse_map(raw);
        let mut num = adj_numbers(num_coords, sym_coords);
        println!("{:?}", num);


        let mut num_test = vec![467, 35, 633, 617, 592, 755, 664, 598];
        num.sort(); 
        num_test.sort();

        assert_eq!(num, num_test);
    }

    #[test]
    fn gear_numbers_test() {
        let raw = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let (num_coords, sym_coords) = parse_map_to_gear(raw);
        let mut num = gear_numbers(num_coords, sym_coords);
        println!("{:?}", num);


        let mut num_test = vec![16345, 451490];
        num.sort(); 
        num_test.sort();

        assert_eq!(num, num_test);
    }
}