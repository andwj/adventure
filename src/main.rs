//
// A very basic text adventure
//
// by Andrew Apted, 2017.
//
// this code is licensed as CC0 (i.e. public domain).
//

use std::io;

struct World {
    solved: bool,
}

impl World {
    fn new() -> World {
        World {
            solved: false,
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
    Quit,
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

    // art thou a quitter?
    match words[0].as_str() {
        "exit" => return Parse::Quit,
        "quit" => return Parse::Quit,
        _      => (),
    }

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
            "win" => {
                self.solved = true;
                solved_msg();
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

    while ! world.solved {
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
            Parse::Quit     => { quit_msg(); break; },
        }
    }
}

//--- editor settings ---
// vi:ts=4:sw=4:expandtab
