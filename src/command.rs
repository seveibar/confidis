use fasthash::{metro};
use std::fmt;

#[derive(Debug)]
pub enum CommandType {
    Set,
    Get
}

#[derive(Debug)]
pub struct Command {
    pub cmd: CommandType,
    pub source: String,
    pub distribution: String,
    pub question: String,
    pub answer: String
}

#[derive(Debug)]
pub struct Answer {
    pub hash: u64,
    pub content: String,
    pub source: String
}

impl Answer {
    pub fn new(content: String, source: String) -> Self {
        Answer {
            hash: metro::hash64(content.as_bytes()),
            content: content,
            source: source
        }
    }
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}