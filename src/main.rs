//
// A very simple text adventure
//
// by Andrew Apted, 2017.
//
// this code is licensed as CC0 (i.e. public domain).
//

use std::io;
use std::io::Write;
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
    N, S, E, W,
    U, D, IN, OUT
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
            rooms: World::create_rooms(),
            location: Mountain,
        }
    }

    fn create_rooms() -> HashMap<RoomId,Room> {
        let mut rm = HashMap::new();

        rm.insert(Mountain,
            Room {
                description: "You are standing on a large grassy mountain.\nTo the north you see a thick forest.\nOther directions are blocked by steep cliffs.",
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

    fn describe_room(&mut self) {
        let room = self.rooms.get(&self.location).unwrap();

        println!("{}", room.description);

        // TODO : show items / monsters
    }
}

fn intro_msg() {
    println!("Welcome to a very simple adventure game!");
    println!("");
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

fn unwrap_str<'a>(w: Option<&'a String>) -> &'a str {
    match w {
        Some(s) => s.as_str(),
        None    => ""
    }
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
    fn parse_command(&mut self, words: &Vec<String>) {
        // we will access the words using an iterator
        let mut words = words.iter();

        let cmd = unwrap_str(words.next());

        if cmd == "" {
            println!("Huh??");
            return;
        }

        // possible nouns (etc)
        let noun1 = unwrap_str(words.next());
        let noun2 = unwrap_str(words.next());

        match cmd {
            "help" => self.cmd_help(),

            "exit" | "quit" | "q" => self.cmd_quit(),

            "i" | "inv" | "invent" | "inventory" => self.cmd_invent(),

            "look" => self.cmd_look(),

            "go" | "walk" => self.cmd_go(noun1),

            "n"  | "north" | "s"  | "south" |
            "e"  | "east"  | "w"  | "west"  |
            "in" | "out"   | "up" | "down" => self.cmd_go(cmd),

            "drop" => self.cmd_drop(noun1),

            "get" | "take" => self.cmd_get(noun1),

            "give" | "offer" => self.cmd_give(noun1, noun2),

            "kill" | "attack" | "hit" | "fight" => self.cmd_kill(noun1),

            "open" | "unlock" => self.cmd_open(noun1),

            "use"  | "apply" => self.cmd_use(noun1),

            _ => {
                println!("I don't understand '{}'", cmd);
            }
        }
    }

    /* implementation of each command */

    fn cmd_help(&mut self) {
        // TODO
    }

    fn cmd_quit(&mut self) {
        quit_msg();
        self.game_over = true;
    }

    fn cmd_invent(&mut self) {
        // TODO
    }

    fn cmd_look(&mut self) {
        // TODO
    }

    fn cmd_go(&mut self, noun1: &str) {
        // TODO
    }

    fn cmd_drop(&mut self, noun1: &str) {
        // TODO
    }

    fn cmd_get(&mut self, noun1: &str) {
        // TODO
    }

    fn cmd_give(&mut self, noun1: &str, noun2: &str) {
        // TODO
    }

    fn cmd_kill(&mut self, noun1: &str) {
        // TODO
    }

    fn cmd_open(&mut self, noun1: &str) {
        // TODO
    }

    fn cmd_use(&mut self, noun1: &str) {
        // TODO
    }
}

fn main() {
    intro_msg();

    let mut world = World::new();

    world.describe_room();

    while ! world.game_over {
        // read a command
        print!("> ");
        io::stdout().flush();

        let mut input = String::new();

        io::stdin().read_line(&mut input)
                   .expect("Error reading input!");

        // parse the command into words
        let parse = parse_input(&input);

        match parse {
            Parse::Empty    => /* ignore a blank line */ (),
            Parse::Bad      => /* parser said why */ (),
            Parse::Words(w) => world.parse_command(&w)
        }
    }
}

//--- editor settings ---
// vi:ts=4:sw=4:expandtab
