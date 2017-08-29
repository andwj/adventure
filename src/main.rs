//
// A very basic text adventure
//
// by Andrew Apted, 2017.
//
// this code is licensed as CC0 (i.e. public domain).
//

use std::io;

struct World {
    game_over: bool,
}

impl World {
    fn new() -> World {
        World {
            game_over: false,
        }
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

    s
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
