mod cluster;
mod command;
mod equalifier;
mod graph;

// use std::io;
use command::{Command, CommandType};
use std::env;
use std::fs;

fn convert_to_command(line: &str) -> Command {
    let items: Vec<&str> = line.split_whitespace().collect();
    match items[0] {
        "SET" | "set" => {
            // SET <question> <answer> FROM <source>
            return Command {
                cmd: CommandType::Set,
                question: String::from(items[1]),
                distribution: String::from("default"),
                answer: String::from(items[2]),
                source: String::from(items[4]),
            };
        }
        "GET" | "get" => {
            // GET <question>
            return Command {
                cmd: CommandType::Get,
                question: String::from(items[1]),
                source: String::from(""),
                answer: String::from(""),
                distribution: String::from("default"),
            };
        }
        _ => panic!("Invalid command: {}", items[0]),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Couldn't read file");

    let lines = contents.lines().filter(|line| !line.is_empty());

    let commands: Vec<Command> = lines.map(|line| convert_to_command(line)).collect();

    let mut g = graph::Graph::new();

    for command in &commands {
        let output = g
            .execute_command(&command)
            .expect("Couldn't execute command");

        println!("{}", output);
    }

    // println!("{:?}", commands);
}
