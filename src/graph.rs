use crate::command::{Command, CommandType, Answer};
use crate::cluster::compute_clusters;

use std::collections::HashMap;
use std::result::Result;

type SourceId = String;
type QuestionId = String;

#[derive(Debug)]
pub struct Source {
    name: SourceId,
    quality: f64,
    strength: i64,
}

#[derive(Debug)]
pub struct Question {
    name: QuestionId,
    answer: Option<Answer>,
    confidence: f64,
    sources: Vec<SourceId>,
    answers: HashMap<SourceId, Answer>
}

#[derive(Debug)]
pub struct Graph {
    sources: HashMap<String, Source>,
    questions: HashMap<String, Question>,
    influence_source_confidence: f64,
    default_source_quality: f64,
    initial_source_strength: i64,
    strength_maximum: i64,
    influence_question_strength: i64,
    answer_count: u64
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            sources: HashMap::new(),
            questions: HashMap::new(),
            influence_source_confidence: 0.7,
            default_source_quality: 0.5,
            initial_source_strength: 1,
            strength_maximum: 100,
            influence_question_strength: 1,
            answer_count: 0
        }
    }
    pub fn execute_command(&mut self, cmd: &Command) -> Result<String, &str> {
        match cmd.cmd {
            CommandType::Set => {
                if !self.sources.contains_key(&cmd.source) {
                    self.sources.insert(cmd.source.to_string(), Source {
                        name: cmd.source.to_string(),
                        quality: self.default_source_quality,
                        strength: self.initial_source_strength
                    });
                }
                if !self.questions.contains_key(&cmd.question) {
                    self.questions.insert(cmd.question.to_string(), Question {
                        name: cmd.question.to_string(),
                        answer: None,
                        confidence: 0.0,
                        sources: vec![],
                        answers: HashMap::new()
                    });
                }
                let mut source = self.sources.get_mut(&cmd.source).unwrap();
                let mut question = self.questions.get_mut(&cmd.question).unwrap();

                // Update source quality if answer has high enough confidence
                if question.answer.is_some() && question.confidence > self.influence_source_confidence {
                    // update source quality
                    // TODO put distance function here & clustering
                    if &cmd.answer == &question.answer.as_ref().unwrap().content {
                        println!("CORRECT: source ({}) matches answer for question ({})", cmd.source, cmd.question);
                        source.quality = (source.quality * source.strength as f64 + 1.0) / (source.strength as f64 + 1.0);
                    } else {
                        println!("WRONG: source ({}) doesn't match answer for question ({})", cmd.source, cmd.question);
                        source.quality = (source.quality * source.strength as f64) / (source.strength as f64 + 1.0);
                    }
                    if source.strength < self.strength_maximum {
                        source.strength += 1;
                    }
                }

                // Compute question clusters
                // let clusterMap = HashMap<u64, >

                question.answers.insert(
                    cmd.source.to_string(),
                    Answer::new(cmd.answer.clone(), cmd.source.clone())
                );

                if source.strength > self.influence_question_strength {
                    // Compute the probability of each answer
                    // HashMap<AnswerId, Vec<Double>>


                    // TODO update connected sources?
                    // for each connected source {
                    //   source.quality = source.quality * (source.strength - 1 + (isCorrect ? 1 : 0)) / source.strength
                    // }
                }

                // Update question if it has enough sources
                // TODO


                return Ok(String::from(""))
            }
            CommandType::Get => {
                let question:&Question = self.questions.get(&cmd.question).unwrap();
                let default_answer: Answer = Answer::new(String::from("None"), String::from(""));
                Ok(format!("{} {} {}", question.name, question.confidence, question.answer.as_ref().unwrap_or(&default_answer)))
                // return Ok(String::from("Hi"))
            }
        }
    }
}
