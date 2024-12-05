pub struct AoC;
const KEYS: [char; 3] = ['M', 'A', 'S'];

type WordMatch = fn(usize, usize, usize, &Vec<Vec<char>>) -> bool;

impl advent_of_rust::Day for AoC {
    fn p1(&self, data: String) {
        let input = data_parser(data);
        let count = xmas_finder(input);
        dbg!(count);
    }
    
    fn p2(&self, data: String) {
        let input = data_parser(data);
        let res = x_mas_finder(input);
        dbg!(res);
    }
}

fn data_parser(raw: String) -> Vec<Vec<char>> {
    raw.lines().map(|line| line.chars().collect()).collect()
}

fn xmas_finder(data: Vec<Vec<char>>) -> usize {
    let matcher: Vec<WordMatch> = vec![
        vertical_up,
        vertical_down,
        horizontal_right,
        horizontal_left,
        diagonal_right_up,
        diagonal_left_up,
        diagonal_right_down,
        diagonal_left_down,
    ];

    let mut count = 0;
    for (i, line) in data.iter().enumerate() {
        for(j, c) in line.iter().enumerate() {

            if *c == 'X' {
                let matched = matcher.iter()
                    .filter(|func| func(i, j, 0, &data))
                    .count();

                dbg!(&matched, &i, &j);
                count += matched;
            }
        }
    }

    return count
}

fn vertical_up(i: usize, j: usize, p: usize, data: &Vec<Vec<char>>) -> bool {
    if i != 0 {
        if data[i-1][j] == KEYS[p] {
            if p == KEYS.len() - 1 {
                return true
            }

            return vertical_up(i-1, j, p+1, data)
        }
    }

    return false
}

fn vertical_down(i: usize, j: usize, p: usize, data: &Vec<Vec<char>>) -> bool {
    if i != data.len()-1 {
        if data[i+1][j] == KEYS[p] {
            if p == KEYS.len()-1 {
                return true
            }

            return vertical_down(i+1, j, p+1, data)
        }
    }
    return false
}

fn horizontal_right(i: usize, j: usize, p: usize, data: &Vec<Vec<char>>) -> bool {
    if j != data[0].len()-1 {
        if data[i][j+1] == KEYS[p] {
            if p == KEYS.len()-1 {
                return true
            }

            return horizontal_right(i, j+1, p+1, data)
        }
    }
    return false
}

fn horizontal_left(i: usize, j: usize, p: usize, data: &Vec<Vec<char>>) -> bool {
    if j != 0 {
        if data[i][j-1] == KEYS[p] {
            if p == KEYS.len()-1 {
                return true
            }

            return horizontal_left(i, j-1, p+1, data)
        }
    }
    return false
}

fn diagonal_right_up(i: usize, j: usize, p: usize, data: &Vec<Vec<char>>) -> bool {
    if i != 0 && j != data[0].len()-1{
        if data[i-1][j+1] == KEYS[p] {
            if p == KEYS.len()-1 {
                return true
            }

            return diagonal_right_up(i-1, j+1, p+1, data)
        }
    }
    return false
}

fn diagonal_left_up(i: usize, j: usize, p: usize, data: &Vec<Vec<char>>) -> bool {
    if i != 0 && j != 0{
        if data[i-1][j-1] == KEYS[p] {
            if p == KEYS.len()-1 {
                return true
            }

            return diagonal_left_up(i-1, j-1, p+1, data)
        }
    }
    return false
}

fn diagonal_right_down(i: usize, j: usize, p: usize, data: &Vec<Vec<char>>) -> bool {
    if i != data.len()-1 && j != data[0].len()-1{
        if data[i+1][j+1] == KEYS[p] {
            if p == KEYS.len()-1 {
                return true
            }

            return diagonal_right_down(i+1, j+1, p+1, data)
        }
    }
    return false
}

fn diagonal_left_down(i: usize, j: usize, p: usize, data: &Vec<Vec<char>>) -> bool {
    if i != data.len()-1 && j != 0{
        if data[i+1][j-1] == KEYS[p] {
            if p == KEYS.len()-1 {
                return true
            }

            return diagonal_left_down(i+1, j-1, p+1, data)
        }
    }
    return false
}


fn valid_index(i: usize, max: usize) -> bool {
    !(i == 0 || i+1 >= max)
}

fn x_mas_finder(data: Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for (i, line) in data.iter().enumerate() {
        for(j, c) in line.iter().enumerate() {

            if *c == 'A'  && valid_index(i, data.len()) && valid_index(j, line.len()) {

                let l1 = [data[i-1][j-1], data[i+1][j+1]];
                let l2 = [data[i+1][j-1], data[i-1][j+1]];

                if l1.contains(&'M') && l1.contains(&'S') && l2.contains(&'M') && l2.contains(&'S'){
                    count += 1;
                }             
            }
        }
    }

    return count
}

mod tests {
    use super::*;

    #[test]
    fn map_creation() {
        let raw = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let data = data_parser(raw.to_string());
        let count = xmas_finder(data);
        assert_eq!(18, count);
    }

    #[test]
    fn difference_test() {
        let raw = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let input = data_parser(raw.to_string());
        let res = x_mas_finder(input);
        assert_eq!(res, 9)
    }

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