use std::collections::HashMap;

pub struct AoC;

impl advent_of_rust::Day for AoC {
    fn p1(&self, data: String) {
        let parsed = data_parser(&data);
        let acc = halting_problem(parsed);
        println!("{}", acc);
    }
    
    fn p2(&self, data: String) {
        let parsed = data_parser(&data);
        let acc = brute_force(&parsed);
        println!("{}", acc.unwrap());
    }
}


fn data_parser(data: &str) -> Vec<(&str, i32)>{
    data.lines()
        .map(|x| (&x[..3], 
                       x[4..].replace("+", "")
                             .parse::<i32>()
                             .unwrap()))
        .collect::<Vec<(&str, i32)>>()
}


fn halting_problem(data: Vec<(&str, i32)>) -> i32 {
    let mut index: i32 = 0;
    let mut computed = Vec::<i32>::new();
    let mut acc = 0;

    while !computed.contains(&index) {
        computed.push(index);

        match &data[index as usize] {
            ("acc", i) => { acc += i; index += 1; println!("acc {} {}", &i, &index);},
            ("jmp", i) => { index += i; println!("jmp {} {}", &i, &index);},
            ("nop", i) => { index += 1; println!("nop {} {}", &i, &index);},
            (_, _) => { return 0 }
        }

        println!("{:?}", &computed);
    }

    acc
}


fn computer(switch: i32, data: &Vec<(&str, i32)>) -> (bool, i32) {
    let mut acc = 0;
    let mut index = 0;
    let mut computed = Vec::<i32>::new();

    loop {
        computed.push(index);

        if index == switch {
            println!("{} {:?}", &switch, data);
            match data[index as usize] {
                ("jmp", i) => { index += 1; },
                ("nop", i) => { index += i;},
                (_, _) => { panic!("what??") }
            }
        }

        match data[index as usize] {
            ("acc", i) => { acc += i; index += 1;},
            ("jmp", i) => { index += i; },
            ("nop", i) => { index += 1; },
            (_, _) => { panic!("what??") }
        }

        if computed.contains(&index) || index as usize > data.len() || index < 0 {
            return (false, acc)
        }

        if index as usize == (data.len()) {
            return (true, acc)
        }
    }
}


fn brute_force(data: &Vec<(&str, i32)>) -> Option<i32> {

    let bit_flip = ["jmp", "nop"];

    for (i, (k, v)) in data.iter().enumerate() {
        if  bit_flip.contains(k) && 
            v != &1 && 
            v != &0 && 
            (i as i32 + v ) < data.len() as i32 && 
            (i as i32 + v ) >= 0 
        {
            let (res, acc) = computer(i as i32, data);

            if res {
                return Some(acc)
            }
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_parsed_data() {
        let data = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        let res: Vec<(&str, i32)> = Vec::from([
            ("nop", 0),
            ("acc", 1),
            ("jmp", 4),
            ("acc", 3),
            ("jmp", -3),
            ("acc", -99),
            ("acc", 1),
            ("jmp", -4),
            ("acc", 6),
        ]);

        assert_eq!(res, data_parser(data));
    }

    #[test]
    fn p1_halt() {
        let data: Vec<(&str, i32)> = Vec::from([
            ("nop", 0),
            ("acc", 1),
            ("jmp", 4),
            ("acc", 3),
            ("jmp", -3),
            ("acc", -99),
            ("acc", 1),
            ("jmp", -4),
            ("acc", 6),
        ]);

        assert_eq!(5, halting_problem(data));
    }

    #[test]
    fn p2_halt() {
        let data: Vec<(&str, i32)> = Vec::from([
            ("nop", 0),
            ("acc", 1),
            ("jmp", 4),
            ("acc", 3),
            ("jmp", -3),
            ("acc", -99),
            ("acc", 1),
            ("jmp", -4),
            ("acc", 6),
        ]);

        assert_eq!(8, brute_force(&data).unwrap());
    }
}
