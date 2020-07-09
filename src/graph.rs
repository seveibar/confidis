use crate::command::{Command, CommandType};

use std::collections::HashMap;
use std::result::Result;

pub struct Source {
    quality: f64,
    strength: i64,
}

pub struct Question {
    answer: Option<String>,
    confidence: f64,
    sources: Vec<String>,
    answers: HashMap<String, String>
}

pub struct Graph {
    sources: HashMap<String, Source>,
    questions: HashMap<String, Question>,
    influence_source_confidence: f64,
    default_source_quality: f64,
    initial_source_strength: i64
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            sources: HashMap::new(),
            questions: HashMap::new(),
            influence_source_confidence: 0.7,
            default_source_quality: 0.5,
            initial_source_strength: 1
        }
    }
    pub fn execute_command(&mut self, cmd: &Command) -> Result<String, &str> {
        match cmd.cmd {
            CommandType::Set => {
                if !self.sources.contains_key(&cmd.source) {
                    self.sources.insert(cmd.source.to_string(), Source {
                        quality: self.default_source_quality,
                        strength: self.initial_source_strength
                    });
                }
                if !self.questions.contains_key(&cmd.question) {
                    self.questions.insert(cmd.source.to_string(), Question {
                        answer: None,
                        confidence: 0.0,
                        sources: vec![],
                        answers: HashMap::new()
                    });
                }
                let mut source = self.sources.get_mut(&cmd.source).unwrap();
                let mut question = self.questions.get_mut(&cmd.question).unwrap();

                if question.answer.is_some() && question.confidence > self.influence_source_confidence {
                    // update source quality
                    if cmd.answer == question.answer.unwrap() {
                        // source.quality = (source.quality +
                    }
                }


                return Ok(String::from(""))
            }
            CommandType::Get => {
                // Ok(self.sources.get(cmd.question))
                return Ok(String::from("Hi"))
            }
        }
    }
}
