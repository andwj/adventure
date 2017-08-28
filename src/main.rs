//
// A very basic text adventure
//
// by Andrew Apted, 2017.
//
// this code is licensed as CC0 (i.e. public domain).
//

use std::io;

struct World {
}

impl World {
    fn new() -> World {
        World {
        }
    }

    fn command(&mut self, words: &Vec<String>) {
        // TODO
    }
}

fn intro_msg() {
    println!("Welcome.....");
}

fn quit_msg() {
    println!("Goodbye!");
}

enum Parse {
    Empty,
    Bad,
    Quit,
    Words(Vec<String>)
}

fn parse_input(input: &String) -> Parse {
    let words: Vec<&str> = input.split_whitespace().collect();

    if words.is_empty() {
        return Parse::Empty;
    }

    // TODO Parse::Quit

    let mut words2: Vec<String> = Vec::new();

    for w in words {
        words2.push(String::from(w));
    }

    Parse::Words(words2)
}

fn main() {
    intro_msg();

    let mut world = World::new();

    loop {
        // read a command
        let mut input = String::new();

        io::stdin().read_line(&mut input)
                   .expect("Error reading input!");

        // parse the command into words
        let parse = parse_input(&input);

        match parse {
            Parse::Empty => /* ignore a blank line */ (),
            Parse::Bad   => /* parser said why */ (),
            Parse::Quit  => { quit_msg(); break; }
            Parse::Words(w) => /* send command to world */ world.command(&w)
        }
    }
}

//--- editor settings ---
// vi:ts=4:sw=4:expandtab
