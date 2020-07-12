mod cluster;
mod command;
mod equalifier;
mod graph;

// use std::io;
use command::{Command, CommandType};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Couldn't read file");

    let lines = contents.lines().filter(|line| !line.is_empty());

    let commands: Vec<Command> = lines.map(|line| Command::from(line)).collect();

    let mut g = graph::Graph::new();

    for command in &commands {
        let output = g
            .execute_command(&command)
            .expect("Couldn't execute command");

        println!("{}", output);
    }

    // println!("{:?}", commands);
}
