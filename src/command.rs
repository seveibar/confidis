use fasthash::metro;
use std::fmt;

#[derive(Debug)]
pub enum CommandType {
    Set,
    Get,
}

#[derive(Debug)]
pub struct Command {
    pub cmd: CommandType,
    pub source: String,
    pub distribution: String,
    pub question: String,
    pub answer: String,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.cmd {
            CommandType::Set => write!(
                f,
                "SET {} {} FROM {}",
                self.question, self.answer, self.source
            ),
            CommandType::Get => write!(f, "GET {}", self.question),
        }
    }
}

impl Command {
    pub fn from(line: &str) -> Command {
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
}

#[derive(Debug, Clone)]
pub struct Answer {
    pub hash: u64,
    pub content: String,
    pub source: String,
}

impl Answer {
    pub fn new(content: String, source: String) -> Self {
        Answer {
            hash: metro::hash64(content.as_bytes()),
            content: content,
            source: source,
        }
    }
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}
