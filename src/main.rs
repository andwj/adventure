//
// A very basic text adventure
//
// by Andrew Apted, 2017.
//
// this code is licensed as CC0 (i.e. public domain).
//

use std::io;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
enum RoomId {
    Mountain,
    Forest,
    Lake,
    Outside,  // of the castle
    Castle,   // inside it
}

use RoomId::*;

#[derive(PartialEq, Eq, Hash)]
enum Dir {
    N, S, E, W
}

enum Lock {
    None,
    Key,
    Dragon
}

struct Exit {
    dir: Dir,
    dest: RoomId,
    lock: Lock,
}

impl Exit {
    fn new(dir: Dir, dest: RoomId, lock: Lock) -> Exit {
        Exit { dir, dest, lock }
    }
}

struct Room {
    description: &'static str,
    exits: Vec<Exit>,
}

struct World {
    game_over: bool,
    rooms: HashMap<RoomId,Room>,
    location: RoomId,
}

impl World {
    fn new() -> World {
        World {
            game_over: false,
            rooms: World::populate_rooms(),
            location: Mountain,
        }
    }

    fn populate_rooms() -> HashMap<RoomId,Room> {
        let mut rm = HashMap::new();

        rm.insert(Mountain,
            Room {
                description: "You are standing on a large grassy mountain.\nTo the north you see a thick forest.Other directions are blocked by steep cliffs.",
                exits: vec![
                    Exit::new( Dir::N, Forest, Lock::None),
                ],
            });

        rm.insert(Forest,
            Room {
                description: "You are in a forest, surrounded by dense trees and shrubs.\nA wide path spirals upwards to the south, whereas narrow\npaths lead east and west.",
                exits: vec![
                    Exit::new( Dir::S, Mountain, Lock::None),
                    Exit::new( Dir::W, Lake,     Lock::None),
                    Exit::new( Dir::E, Outside,  Lock::None),
                ],
            });

        rm
    }
}

fn intro_msg() {
    println!("Welcome.....");
}

fn quit_msg() {
    println!("Goodbye!");
}

fn solved_msg() {
    println!("Congratulations, you have won!");
}

enum Parse {
    Empty,
    Bad,
    Words(Vec<String>),
}

fn sanitize_word(word: &str) -> String {
    let mut s = String::new();

    // convert to lowercase
    for c in word.chars() {
        for d in c.to_lowercase() {
            s.push(d);
        }
    }

    // ignore certain words
    match s.as_str() {
        "a" | "an" | "the" | "to" => String::new(),
        _ => s
    }
}

fn sanitize_list(words: &Vec<&str>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for w in words {
        let s = sanitize_word(w);

        if s != "" {
            result.push(s);
        }
    }

    result
}

fn parse_input(input: &String) -> Parse {
    let words: Vec<&str> = input.split_whitespace().collect();

    if words.is_empty() {
        return Parse::Empty;
    }

    let words = sanitize_list(&words);

    Parse::Words(words)
}

impl World {
    fn command(&mut self, words: &Vec<String>) {
        if words.is_empty() {
            println!("Huh??");
            return;
        }

        let cmd = words[0].as_str();

        match cmd {
            "exit" | "quit" => {
                quit_msg();
                self.game_over = true;
            },

            // FIXME : test stuff, REMOVE THIS
            "win" => {
                solved_msg();
                self.game_over = true;
            },

            _ => {
                println!("I don't understand '{}'", cmd);
            }
        }
    }
}

fn main() {
    intro_msg();

    let mut world = World::new();

    while ! world.game_over {
        // read a command
        let mut input = String::new();

        io::stdin().read_line(&mut input)
                   .expect("Error reading input!");

        // parse the command into words
        let parse = parse_input(&input);

        match parse {
            Parse::Empty    => /* ignore a blank line */ (),
            Parse::Bad      => /* parser said why */ (),
            Parse::Words(w) => /* send command to world */ world.command(&w),
        }
    }
}

//--- editor settings ---
// vi:ts=4:sw=4:expandtab
