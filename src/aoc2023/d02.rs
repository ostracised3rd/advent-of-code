

use std::collections::HashMap;

use prse::parse;
use tracing::{debug, info};
pub struct AoC;

impl advent_of_rust::Day for AoC {
    fn p1(&self, data: String) {
        let games = parse_input(&data);
        info!("{games:?}");

       let rule = HashMap::from([
        ("red".to_string(), 12 as i64),
        ("green".to_string(), 13 as i64),
        ("blue".to_string(), 14 as i64),
       ]);


       let sum = games.iter()
                .filter(|game| validity(*game, &rule))
                .fold(0, |init, game| init + game.id);

        info!("{sum}");
    }
    
    fn p2(&self, data: String) {
        let games = parse_input(&data);
        let sum: i64 = games.iter().map(|game| minimum_set(game)).sum();
        info!("{sum}");
    }
}

fn minimum_set(game: &Game) -> i64 {
    let set = game.rounds.iter().fold((0, 0, 0), |mut init, round| {
        for (k, v) in round {
            if k == "green" {
                init.0 = i64::max(init.0, *v);
            } else if k == "blue" {
                init.1 = i64::max(init.1, *v);
            } else if k == "red" {
                init.2 = i64::max(init.2, *v);
            }
        }

        init
    });

    set.0 * set.1 * set.2
}

fn validity(game: &Game, rule: &HashMap<String, i64>) -> bool {
    for round in game.rounds.iter() {
        for (k, v) in round.iter() {
            if rule[k] < *v {
                return false
            }
        }
    }

    return true
}

fn parse_input(raw: &str) -> Vec<Game> {
    raw.lines()
        .map(|line| parse_game(line)).collect()
}

fn parse_game(row: &str) -> Game {
    let (game, data) = row.split_once(":").unwrap();
    Game {
        id: game.replace("Game ", "").parse().unwrap(),
        rounds: data.split(";").map(|game| parse_round(game)).collect()
    }
}

fn parse_round(game: &str) -> HashMap<String, i64> {
    game.split(",").map(|cube| {
        let (count, color): (&str, &str) = parse!(cube, " {} {}");
        debug!("{color}: {count}");
        let count: i64 = count.parse().unwrap();
        (color.to_string(), count)
    }).collect::<HashMap<String, i64>>()
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: i64,
    rounds: Vec<HashMap<String, i64>>
}


mod tests {
    use super::*;


    #[test]
    fn game_test() {
        let raw = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let data = vec![
            Game {
                id: 1, 
                rounds: vec![
                    HashMap::<String, i64>::from([("blue".to_string(), 3 as i64), ("red".to_string(), 4 as i64)]),
                    HashMap::<String, i64>::from([("red".to_string(), 1 as i64), ("green".to_string(), 2 as i64), ("blue".to_string(), 6 as i64)]),
                    HashMap::<String, i64>::from([("green".to_string(), 2 as i64)])
                ],
            },
        ];

        assert_eq!(data, parse_input(raw))
    }

    #[test]
    fn validity_test() {
        let raw = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let games = parse_input(raw);
        let rule = HashMap::from([
            ("red".to_string(), 12 as i64),
            ("green".to_string(), 13 as i64),
            ("blue".to_string(), 14 as i64),
        ]);

        let sum = games.iter()
                .filter(|game| validity(*game, &rule))
                .fold(0, |init, game| init + game.id);

        assert_eq!(8, sum);
    }

    #[test]
    fn minimum_set_test() {
        let raw = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let games = parse_input(raw);

        let sum: i64 = games.iter().map(|game| minimum_set(game)).sum();

        assert_eq!(2286, sum);
    }
}