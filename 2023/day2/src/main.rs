#![feature(iter_advance_by)]

use core::num;
use std::{ fs::{ read_to_string }, io::{}, default, collections::{ HashMap, HashSet }, usize };

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

#[derive(Debug)]
enum BagState {
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

#[derive(Default)]
struct Game {
    id: usize,
    moves: Vec<Vec<Cube>>,
}

fn main() {
    println!("Starting..");
    // let games: Vec<Game> = read_to_string("test1.txt")
    //     .unwrap()
    //     .lines()
    //     .map(|line| { parse_file(&String::from(line)) })
    //     .collect();
}

fn parse_file(game_string: &String) -> Game {
    let mut moves_vec: Vec<Vec<Cube>> = Vec::new();

    // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let (colon, space) = (
        game_string.find(|c: char| c == ':').unwrap(),
        game_string.find(|c: char| c == ' ').unwrap(),
    );

    let game_idx = &game_string[space..colon].trim().to_string();
    println!("Game {game_idx}");
    let move_string: &str = &game_string[colon + 2..game_string.len()].trim();
    println!("Move {move_string}");

    for moves in move_string.split(';') {
        for cubes in moves.split(';') {
            let mut cubez: Vec<Cube> = Vec::new();
            for cube in cubes.split(',') {
                let cube_info: Vec<&str> = cube.trim().split(' ').collect();
                println!("Cube info: {:?}", &cube_info);
                let color = cube_info.last().unwrap().trim();
                let count = cube_info.first().unwrap().trim();
                match color {
                    "blue" =>
                        cubez.push(Cube::Blue { count: count.to_string().parse::<i8>().unwrap() }),
                    "red" =>
                        cubez.push(Cube::Red { count: count.to_string().parse::<i8>().unwrap() }),
                    "green" =>
                        cubez.push(Cube::Green { count: count.to_string().parse::<i8>().unwrap() }),
                    _ => {
                        panic!("{color} Not found ");
                    }
                }
            }
            moves_vec.push(cubez);
        }
    }

    return Game {
        id: game_idx.parse::<usize>().unwrap(),
        moves: moves_vec,
    };
}

fn game_idx(game_string: String) -> String {
    let idx = game_string.split(' ').last().unwrap();
    return idx.to_string();
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
        let moves = game.moves.first().unwrap();
        let first_mv = moves.first().unwrap();
        println!("{:?}", moves[0]);

        assert_eq!(game.id, 12);
        assert!(first_mv == &(Cube::Blue { count: 3 }));
        assert!(moves[1] == Cube::Red { count: 4 });
    }
}
