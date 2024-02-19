use std::{collections::{HashMap, VecDeque}, hash::Hash};

use prse::parse;
use tracing::info;

pub struct AoC;

impl advent_of_rust::Day for AoC {
    fn p1(&self, data: String) {
        let cards = parser(&data);
        let points: Vec<i64> = cards.iter().map(|(_, card)| card.calc_point()).collect();
        let res: i64 = points.iter().sum();
        info!(res);
    }
    
    fn p2(&self, data: String) {
        let cards = parser(&data);
        let count = card_processor(cards);
        info!(count);
    }
}


#[derive(Debug)]
struct Card {
    id: i64,
    numbers: Vec<i64>,
    winners: HashMap::<i64, bool>,
}


impl Card {
    fn new(line: &str) -> Self {
        let (id, numbers, winners): (&str, &str, &str) = parse!(line, "Card {}: {} | {}");
        let id: i64 = id.trim().parse().unwrap(); 

        let numbers: Vec<i64> = numbers.split(" ")
            .filter(|i| i.parse::<i64>().is_ok())
            .map(|i| i.parse().unwrap())
            .collect();

        let winners: HashMap::<i64, bool> = winners.split(" ")
            .filter(|i| i.parse::<i64>().is_ok())
            .map(|i| (i.parse().unwrap_or(0), false))
            .collect();


        Card {
            id,
            numbers,
            winners,
        }
    }

    fn matching(&self) -> i64 {
        self.numbers.iter().filter(|i| self.winners.contains_key(i)).collect::<Vec<&i64>>().len() as i64
    }

    fn calc_point(&self) -> i64 {
        let point = self.matching() - 1;
        if point >= 0 {(2 as i64).pow(point as u32)} else {0}
    }
}


fn parser(raw: &str) -> HashMap<i64, Card> {
    raw.lines().map(|l| {
        let card = Card::new(l);
        (card.id, card)
    }).collect()
}

fn card_processor(cards: HashMap<i64, Card>) -> i64 {
    let mut queue: VecDeque::<i64> = cards.iter().map(|(k, _)| *k).collect();
    let mut count = cards.len() as i64;
    let last = cards.len() as i64;
    let mut counter = 0;

    loop {
        let step = queue.pop_front();
        // println!("queue: {} => {}: {}: {:?}", counter, count, queue.len(), &queue);
        match step {
            Some(id) => {
                let c = cards.get(&id).unwrap().matching();
                for i in 1..c+1 {
                    if id+i > last {break}

                    queue.push_back(id+i);
                    count += 1;
                }
            },
            None => break
        } 

        counter += 1;
    }

    println!("queue: {}", counter);
    count
}


mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let raw = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let cards = parser(raw);
        println!("cards {:?}", cards);
        let points: Vec<i64> = cards.iter().map(|(_, card)| card.calc_point()).collect();
        println!("points {:?}", points);
        let tests: Vec<i64> = vec![8, 2, 2, 1, 0, 0];
        assert_eq!(points, tests);

    }

    #[test]
    fn adj_numbers_test() {
        let raw = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let cards = parser(raw);
        let count = card_processor(cards);

        assert_eq!(count, 30);
    }

}