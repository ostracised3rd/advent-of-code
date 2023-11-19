pub struct AoC;

impl advent_of_code::Day for AoC {
    fn p1(&self, data: String) {
        let res = parse_data(&data, p1_group_parse);
        println!("{}", res);
    }
    
    fn p2(&self, data: String) {
        let res = parse_data(&data, p2_group_parse);
        println!("{}", res);
    }
}


fn p1_group_parse(group: &str) -> usize {
    let mut group = group
        .split_whitespace()
        .collect::<String>()
        .chars()
        .collect::<Vec<char>>();

    group.sort();
    group.dedup();
    group.len()
}


fn p2_group_parse(group: &str) -> usize {
    let group = group
        .split_whitespace()
        .collect::<Vec<&str>>();

    let l = group.len();

    let mut group = group
        .iter()
        .flat_map(|s| s.chars())
        .collect::<Vec<char>>();

    if l == 1 { return group.len() }

    group.sort();

    group
        .iter()
        .fold((0, 0, &group[0]), |(n, mut i, c), x| {
            if c == x { i += 1 } else { return (n, 1, x)}
            if i == l { (n+1, i, c) } else { (n, i, c) }
        }).0
}


fn parse_data(data: &str, func: fn(&str) -> usize) -> usize {
    data.split("\n\n").collect::<Vec<&str>>()
        .iter().map(|x| func(x))
        .collect::<Vec<usize>>()
        .iter()
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_answer_check() {
        let data = "abc

a
b
c

ab
ac

a
a
a
a

b";

        assert_eq!(11, parse_data(data, p1_group_parse));
    }

    #[test]
    fn p2_answer_check() {
        let data = "abc

a
b
c

ab
ac

a
a
a
a

b";

        assert_eq!(6, parse_data(data, p2_group_parse));
    }
}