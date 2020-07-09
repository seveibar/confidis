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
