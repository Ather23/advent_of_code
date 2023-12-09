#![feature(iter_advance_by)]

use core::num;
use std::{
    fs::{ read_to_string },
    io::{},
    default,
    collections::{ HashMap, HashSet },
    usize,
    cmp::min,
};

#[derive(Debug, PartialEq)]
enum Cube {
    Red {
        count: i8,
    },
    Green {
        count: i8,
    },
    Blue {
        count: i8,
    },
}

struct Bag {
    bag: HashMap<String, u64>,
}

impl Bag {
    pub fn new() -> Self {
        Bag {
            bag: HashMap::new(),
        }
    }

    pub fn insert(&mut self, color: String, new_count: u64) {
        match self.bag.get(&color) {
            Some(count) => {
                if count < &new_count {
                    // println!("Color: {} Inserted {:?}", &color, (count, new_count));
                    self.bag.insert(color, new_count);
                }
            }
            None => {
                self.bag.insert(color, new_count);
            }
        }
    }
}

#[derive(Default)]
struct Game {
    id: usize,
    moves: Vec<Vec<Cube>>,
}

fn main() {
    println!("Starting..");
    let games: u64 = read_to_string("problem1.txt")
        .unwrap()
        .lines()
        .map(|line| { parse_file(&String::from(line)) })
        .sum();

    println!("Answer: {}", games);
}

fn parse_file(game_string: &String) -> u64 {
    let mut moves_vec: Vec<Vec<Cube>> = Vec::new();

    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let (colon, space) = (
        game_string.find(|c: char| c == ':').unwrap(),
        game_string.find(|c: char| c == ' ').unwrap(),
    );

    let game_idx = &game_string[space..colon].trim().to_string();
    let move_string: &str = &game_string[colon + 2..game_string.len()].trim();

    let mut result: u64 = 0;
    let mut bag: Bag = Bag::new();

    for moves in move_string.split(';') {
        for cubes in moves.split(';') {
            for cube in cubes.split(',') {
                let cube_info: Vec<&str> = cube.trim().split(' ').collect();
                let color = cube_info.last().unwrap().trim();
                let count_str = cube_info.first().unwrap().trim();
                let count = count_str.to_string().parse::<u64>().unwrap();
                bag.insert(color.to_string(), count);
            }
        }
    }
    result = bag.bag.values().fold(1, |acc, &value| acc * value);
    // println!("Idx: {} Bag: {:?}", game_idx, &bag.bag);
    // println!("Result: {}", result);

    return result;
}

#[cfg(test)]
mod tests {
    use crate::{ parse_file, Cube };

    #[test]
    fn parse_file_test() {
        let mut test_string = String::from(
            "Game 12: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        );

        let game = parse_file(&test_string);
        assert_eq!(game, 12)
        // let moves = game.moves.first().unwrap();
        // let first_mv = moves.first().unwrap();
        // println!("{:?}", moves[0]);

        // assert_eq!(game.id, 12);
        // assert!(first_mv == &(Cube::Blue { count: 3 }));
        // assert!(moves[1] == Cube::Red { count: 4 });
    }
}
