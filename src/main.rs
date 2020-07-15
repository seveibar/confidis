mod cluster;
mod command;
mod equalifier;
mod graph;

// use std::io;
use command::Command;
use std::fs;
use std::io::{stdin, stdout, BufRead, Write};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    // filepath to execute commands from
    #[structopt(parse(from_os_str))]
    filepath: Option<std::path::PathBuf>,
}

fn main() {
    let args = Cli::from_args();
    let mut g = graph::Graph::new();

    if args.filepath.is_some() {
        let contents = fs::read_to_string(args.filepath.unwrap()).expect("Couldn't read file");

        let lines = contents.lines().filter(|line| !line.is_empty());

        let commands: Vec<Command> = lines
            .map(|line| Command::from(line).expect(&format!("Invalid line: \"{}\"", line)))
            .collect();

        for ref command in &commands {
            let output = g
                .execute_command(command)
                .expect("Couldn't execute command");
            println!("{}", output);
        }
        return;
    }

    // REPL

    print!("> ");
    stdout().flush();
    for ref line in stdin().lock().lines().filter_map(|x| x.ok()) {
        if line.len() > 0 {
            match Command::from(line) {
                Ok(cmd) => match g.execute_command(&cmd) {
                    Ok(result) => {
                        println!("{}", result);
                    }
                    Err(msg) => {
                        println!("Err: {}", msg);
                    }
                },
                Err(msg) => {
                    println!("Invalid Command: \"{}\"\nErr: {}", line, msg);
                }
            }
        }
        print!("> ");
        stdout().flush();
    }
}
