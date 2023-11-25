use std::collections::{HashSet, HashMap};

pub struct AoC;

impl advent_of_rust::Day for AoC {
    fn p1(&self, data: String) {
        let parsed = p1_data_parser(&data);
        let key = "shiny gold";
        let mut bags = HashSet::<String>::new();
        bags.insert(String::from(key));

        let bags = p1_possible_bags(bags, key, parsed);
        println!("{:?} {}", bags, bags.len() - 1);
    }
    
    fn p2(&self, data: String) {
        let parsed = p2_data_parser(&data);
        let count = p2_too_many_bags(String::from("shiny gold"), &parsed);
        println!("{}", count);
    }
}

fn p1_line_parser(line: &str) -> (String, String) {
    let res = line
        .replace(" bags", "")
        .replace(" bag", "")
        .replace(".", "")
        .split(" contain ")
        .map(|x| String::from(x))
        .collect::<Vec<String>>();

    // BagRule {key: Box::new(res[0]), val: Box::new(res[1])}
    (String::from(&res[0]), String::from(&res[1]))
}


fn p1_data_parser(data: &str) -> Vec<(String, String)> {
    data.lines()
        .filter(|x| !x.contains("no other"))
        .map(|x| p1_line_parser(x))
        .collect::<Vec<(String, String)>>()
}


fn p1_possible_bags(mut bags: HashSet<String>, bag: &str, rules: Vec<(String, String)>) -> HashSet<String> {
    let (p1, p2): (Vec<(String, String)>, Vec<(String, String)>) = rules
        .into_iter()
        .partition(|x| x.1.contains(bag));

    for (k, _) in p1 {
        if bags.contains(&k) { continue; }
        bags.insert(k.clone());
        bags.extend(p1_possible_bags( bags.clone(), &k, p2.clone()));
    }
    
    bags
}

fn p2_line_parser(line: &str) -> (String, Vec<(String, i32)>) {
    let res = line
        .replace(" bags", "")
        .replace(" bag", "")
        .replace(".", "")
        .split(" contain ")
        .map(|x| String::from(x))
        .collect::<Vec<String>>();

    let val_parse = |x: &str| {
        let i = &x[0..1];
        let body = &x[2..];

        (String::from(body), i.parse().unwrap())
    };

    let val = if res[1].contains(',') {
        res[1].split(", ").map(|x| val_parse(x)).collect::<Vec<(String, i32)>>()
    } else {
        vec![val_parse(&res[1])]
    };

    (String::from(&res[0]), val)
}


fn p2_data_parser(data: &str) -> HashMap<String, Vec<(String, i32)>> {
    data.lines()
        .filter(|x| !x.contains("no other"))
        .map(|x| p2_line_parser(x))
        .collect::<HashMap<String, Vec<(String, i32)>>>()
}

fn p2_too_many_bags(key: String, rules: &HashMap<String, Vec<(String, i32)>>) -> usize {
    
    if !rules.contains_key(&key) {
        return 0
    }

    let bags = rules[&key].clone();
    
    let mut res = 0;
    for (k, v) in bags {
        let val = v as usize;
        res += val;
        res += p2_too_many_bags(k, rules) * val;
    }

    return res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_parsed_data() {

        let data = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let res = vec![
            (String::from("light red"), String::from("1 bright white, 2 muted yellow")),
            (String::from("dark orange"), String::from("3 bright white, 4 muted yellow")),
            (String::from("bright white"), String::from("1 shiny gold")),
            (String::from("muted yellow"), String::from("2 shiny gold, 9 faded blue")),
            (String::from("shiny gold"), String::from("1 dark olive, 2 vibrant plum")),
            (String::from("dark olive"), String::from("3 faded blue, 4 dotted black")),
            (String::from("vibrant plum"), String::from("5 faded blue, 6 dotted black"))
        ];


        assert_eq!(res, p1_data_parser(data));
    }

    #[test]
    fn p1_possibilities() {
        let data = vec![
            (String::from("light red"), String::from("1 bright white, 2 muted yellow.")),
            (String::from("dark orange"), String::from("3 bright white, 4 muted yellow.")),
            (String::from("bright white"), String::from("1 shiny gold.")),
            (String::from("muted yellow"), String::from("2 shiny gold, 9 faded blue.")),
            (String::from("shiny gold"), String::from("1 dark olive, 2 vibrant plum.")),
            (String::from("dark olive"), String::from("3 faded blue, 4 dotted black.")),
            (String::from("vibrant plum"), String::from("5 faded blue, 6 dotted black."))
        ];

        let mut res: HashSet::<String> = HashSet::new();
        res.insert(String::from("light red"));
        res.insert(String::from("bright white"));
        res.insert(String::from("dark orange"));
        res.insert(String::from("muted yellow"));

        assert_eq!(res, p1_possible_bags( HashSet::<String>::new(), "shiny gold", data))
    }

    #[test]
    fn p2_bag_count() {
        let data = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        let parsed = p2_data_parser(data);

        assert_eq!(126, p2_too_many_bags(String::from("shiny gold"), &parsed));
    }
}
