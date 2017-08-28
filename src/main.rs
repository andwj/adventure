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
}

fn intro() {
    println!("Welcome.....");
}

fn main() {
    intro();

    let mut world = World::new();

    loop {
        // read a command
        let mut input = String::new();

        io::stdin().read_line(&mut input)
                   .expect("Error reading input!");

        // TODO check for quit

        // TODO send command to world
    }
}

//--- editor settings ---
// vi:ts=4:sw=4:expandtab
