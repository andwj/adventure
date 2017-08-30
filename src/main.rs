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

#[derive(Clone, PartialEq, Eq, Hash)]
enum RoomId {
    NONE,     // only used as result of next_room() method

    Mountain,
    Forest,
    Lake,

    Outside,  // of the castle
    Castle,   // inside it
    Treasury
}

use RoomId::*;

#[derive(PartialEq, Eq, Hash)]
enum Dir {
    N, S, E, W, U, D,
}

#[derive(Clone)]
enum Lock {
    NONE,      // travel is not possible at all
    Free,      // travel is possible and has no obstacle
    Key,       // a key is required
    Crocodile, // a monster is blocking the path
    Password,  // need to tell a password
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

struct ObjectList {
    v: Vec<String>
}

impl ObjectList {
    fn new() -> ObjectList {
        ObjectList { v: vec![] }
    }

    fn from(names: &[&str]) -> ObjectList {
        let mut result = Self::new();

        for name in names {
            result.add(name);
        }

        result
    }

    fn add(&mut self, name: &str) {
        self.v.push(String::from(name));
    }

    fn has(&self, name: &str) -> bool {
        for i in 0 .. self.v.len() {
            if self.v[i].as_str() == name {
                return true;
            }
        }

        false
    }

    fn remove(&mut self, name: &str) -> bool {
        for i in 0 .. self.v.len() {
            if self.v[i].as_str() == name {
                self.v.swap_remove(i);
                return true;
            }
        }

        false
    }
}

struct Room {
    description: &'static str,
    exits: Vec<Exit>,
    objects: ObjectList,
}

impl Room {
    fn can_travel(&self, dir: &Dir) -> Lock {
        for e in &self.exits {
            if e.dir == *dir {
                return e.lock.clone();
            }
        }

        Lock::NONE
    }

    fn next_room(&self, dir: &Dir) -> RoomId {
        for e in &self.exits {
            if e.dir == *dir {
                return e.dest.clone();
            }
        }

        RoomId::NONE
    }
}

struct World {
    game_over: bool,
    rooms: HashMap<RoomId,Room>,
    location: RoomId,
    inventory: ObjectList,
    found_key: bool,
}

impl World {
    fn new() -> World {
        World {
            game_over: false,
            rooms: World::create_rooms(),
            location: Mountain,
            inventory: ObjectList::from(&["sword"]),
            found_key: false,
        }
    }

    fn create_rooms() -> HashMap<RoomId,Room> {
        let mut rm = HashMap::new();

        rm.insert(Mountain,
            Room {
                description: "You are standing on a large grassy mountain.\nTo the north you see a thick forest.\nOther directions are blocked by steep cliffs.",
                exits: vec![
                    Exit::new(Dir::N, Forest, Lock::Free),
                ],
                objects: ObjectList::from(&["roast"])
            });

        rm.insert(Forest,
            Room {
                description: "You are in a forest, surrounded by dense trees and shrubs.\nA wide path slopes gently upwards to the south, and\nnarrow paths lead east and west.",
                exits: vec![
                    Exit::new(Dir::S, Mountain, Lock::Free),
                    Exit::new(Dir::W, Lake,     Lock::Free),
                    Exit::new(Dir::E, Outside,  Lock::Crocodile),
                ],
                objects: ObjectList::from(&["crocodile", "parrot"])
            });

        rm.insert(Lake,
            Room {
                description: "LAKE ",  // FIXME
                exits: vec![
                    Exit::new(Dir::E, Forest, Lock::Free),
                ],
                objects: ObjectList::from(&["carrot"])
            });

        rm.insert(Outside,
            Room {
                description: "OUTSIDE CASTLE", // FIXME
                exits: vec![
                    Exit::new(Dir::W, Forest, Lock::Free),
                    Exit::new(Dir::E, Castle, Lock::Key),
                ],
                objects: ObjectList::new()
            });

        rm.insert(Castle,
            Room {
                description:  "INSIDE CASTLE",  // FIXME
                exits: vec![
                    Exit::new(Dir::W, Outside,  Lock::Free),
                    Exit::new(Dir::S, Treasury, Lock::Password),
                ],
                objects: ObjectList::from(&["guard"])
            });

        rm.insert(Treasury,
            Room {
                description:  "TREASURY",  // FIXME
                exits: vec![
                    Exit::new(Dir::N, Castle, Lock::Free),
                ],
                objects: ObjectList::from(&["treasure"])
            });

        rm
    }

    fn describe_room(&self) {
        let room = self.rooms.get(&self.location).unwrap();

        println!("{}", room.description);

        // show items and monsters
        for ob in &room.objects.v {
            println!("There is a {} here.", ob);
        }
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
    println!("");
    println!("With your good health and new-found wealth, you live");
    println!("happily ever after (well... around 50 years or so).");
    println!("");
    println!("Congratulations, you solved the game!");
}

#[allow(dead_code)]
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

            "look" | "l" => self.cmd_look(),

            "go" | "walk" => self.cmd_go(noun1),

            "n"  | "north" | "s"  | "south" |
            "e"  | "east"  | "w"  | "west"  |
            "d"  | "down"  | "u"  | "up"  => self.cmd_go(cmd),

            "drop" => self.cmd_drop(noun1),

            "get" | "take" => self.cmd_get(noun1),

            "give" | "offer" => self.cmd_give(noun1, noun2),

            "feed" => self.cmd_feed(noun1, noun2),

            "kill" | "attack" | "hit" | "fight" => self.cmd_attack(noun1),

            "open" | "unlock" => self.cmd_open(noun1),

            "swim" | "dive"  => self.cmd_swim(),

            "use"  | "apply" => self.cmd_use(noun1),

            _ => {
                println!("I don't understand '{}'", cmd);
            }
        }
    }

    /* implementation of each command */

    fn cmd_help(&mut self) {
        println!("Use text commands to walk around and do things.");
        println!("Some examples:");
        println!("    go north");
        println!("    get the rope");
        println!("    drop the lantern");
        println!("    inventory");
        println!("    unlock door");
        println!("    kill the serpent");
        println!("    quit");
    }

    fn cmd_quit(&mut self) {
        quit_msg();
        self.game_over = true;
    }

    fn cmd_invent(&mut self) {
        println!("You are carrying:");

        if self.inventory.v.is_empty() {
            println!("    nothing.");
        } else {
            for ob in &self.inventory.v {
                println!("    a {}.", ob);
            }
        }
    }

    fn cmd_look(&mut self) {
        println!("");
        self.describe_room();
    }

    fn cmd_go(&mut self, noun1: &str) {
        if noun1 == "" {
            println!("Go where??");
            return;
        }

        let dir : Dir;

        match noun1 {
            "n" | "north" => dir = Dir::N,
            "s" | "south" => dir = Dir::S,
            "e" | "east"  => dir = Dir::E,
            "w" | "west"  => dir = Dir::W,

            "u" | "up"    => dir = Dir::U,
            "d" | "down"  => dir = Dir::D,

            _ => {
                println!("I don't understand that direction.");
                return;
            }
        }

        let room = self.rooms.get(&self.location).unwrap();

        // check for an obstacle...
        let obst = room.can_travel(&dir);

        match obst {
            Lock::Free => (),

            Lock::NONE => {
                println!("You cannot go that way.");
                return;
            },

            Lock::Key => {
                println!("There is a locked door in your way.");
                return;
            },

            Lock::Crocodile => {
                println!("A huge, scary crocodile blocks your path!");
                return;
            },

            Lock::Password => {
                println!("The guard stops you and says \"Hey, you cannot go\nin there unlessy ou tell me the password!\".");
                return;
            }
        }

        self.location = room.next_room(&dir);

        assert!(self.location != RoomId::NONE);

        println!("");
        self.describe_room();
    }

    fn cmd_drop(&mut self, noun1: &str) {
        if noun1 == "" {
            println!("Drop what??");
            return;
        }

        if ! self.inventory.remove(noun1) {
            println!("You are not carrying a {}.", noun1);
            return;
        }

        let mut room = self.rooms.get_mut(&self.location).unwrap();

        room.objects.add(noun1);
        println!("You drop the {}.", noun1);
    }

    fn cmd_get(&mut self, noun1: &str) {

        match noun1 {
            "" => {
                println!("Get what??");
                return;
            },

            "crocodile" => {
                println!("There mere thought of wrestling with that savage beast\nparalyses you with fear!");
                return;
            },

            "parrot" => {
                println!("The parrot nimbly evades your grasp.");
                return;
            },

            "guard" => {
                println!("A momentary blush suggests the guard was flattered.");
                return;
            },

            _ => ()
        }

        {
            let mut room = self.rooms.get_mut(&self.location).unwrap();

            if ! room.objects.remove(noun1) {
                println!("There is no {} here you can take.", noun1);
                return;
            }
        }

        self.inventory.add(noun1);
        println!("You pick up the {}.", noun1);

        if noun1 == "treasure" {
            solved_msg();
            self.game_over = true;
        }
    }

    fn cmd_feed(&mut self, noun1: &str, noun2: &str) {
        if noun1 == "" || noun2 == "" {
            println!("Feed what to whom??");
            return;
        }

        self.cmd_give(noun1, noun2);
    }

    fn cmd_give(&mut self, noun1: &str, noun2: &str) {
        if noun1 == "" {
            println!("Give what??");
            return;
        }

        if noun2 == "" {
            println!("Give to whom??");
            return;
        }

        if ! self.inventory.has(noun1) {
            println!("You can't give a {}, as you don't have one!", noun1);
            return;
        }

        // TODO: a puzzle involving giving something

        println!("Don't be ridiculous!");
    }

    fn cmd_attack(&mut self, noun1: &str) {
        if noun1 == "" {
            println!("Attack what??");
            return;
        }

        // TODO: some funny messages about attacking something

        let have_sword = self.inventory.has("sword");

        if have_sword {
            println!("You swing your sword, but miss!");
        } else {
            println!("You bruise your hand in the attempt.");
        }
    }

    fn cmd_open(&mut self, noun1: &str) {
        if noun1 == "" {
            println!("Open what??");
            return;
        }

        // TODO: a puzzle involving unlocking a door

        println!("You cannot open that!");
    }

    fn cmd_swim(&mut self) {
        match self.location {
            Lake => {
                if self.found_key {
                    println!("You enjoy a nice swim in the lake.");
                } else {
                    println!("You dive into the lake, enjoy paddling around for a while,\nbut at the bottom you discover a rusty old key!");
                    self.found_key = true;
                    self.inventory.add("key");
                }
            },

            Outside => {
                println!("But the moat is full of crocodiles!");
                return;
            },

            _ => {
                println!("There is nowhere to swim here.");
            }
        }
    }

    fn cmd_use(&mut self, noun1: &str) {
        if noun1 == "" {
            println!("Use what??");
            return;
        }

        if ! self.inventory.has(noun1) {
            println!("You don't have any {} to use.", noun1);
            return;
        }

        // TODO: a puzzle involving using something

        println!("You fiddle with your {}, but nothing happens.", noun1);
    }
}

fn main() {
    intro_msg();

    let mut world = World::new();

    world.describe_room();

    while ! world.game_over {
        // display a prompt
        print!("> ");

        #[allow(unused_must_use)] {
            io::stdout().flush();
        }

        // read a command
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
