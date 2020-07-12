use crate::cluster::compute_clusters;
use crate::command::{Answer, Command, CommandType};
use crate::equalifier::{Equalifier, ExactEqualifier};
use std::collections::HashSet;

use std::collections::HashMap;
use std::result::Result;

type SourceId = String;
type QuestionId = String;

#[derive(Debug)]
pub struct Source {
    name: SourceId,
    quality: f64,
    strength: f64,
}

#[derive(Debug)]
pub struct Question {
    name: QuestionId,
    correct_answers: Vec<Answer>,
    weight: f64,
    confidence: f64,
    sources: Vec<SourceId>,
    answers: Vec<Answer>,
}

fn argmaxf(vec: &Vec<f64>) -> usize {
    let mut highest_index = 0_usize;
    let mut highest_value = vec[0];
    for (i, v) in vec.iter().enumerate() {
        if *v > highest_value {
            highest_index = i;
            highest_value = *v;
        }
    }
    return highest_index;
}

pub struct Graph {
    sources: HashMap<String, Source>,
    questions: HashMap<String, Question>,
    influence_source_confidence: f64,
    default_source_quality: f64,
    initial_source_strength: f64,
    strength_maximum: i64,
    influence_question_strength: i64,
    answer_count: u64,
    equalifier: Box<dyn Equalifier>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            sources: HashMap::new(),
            questions: HashMap::new(),
            influence_source_confidence: 0.7,
            default_source_quality: 0.5,
            initial_source_strength: 1.0,
            strength_maximum: 100,
            influence_question_strength: 1,
            answer_count: 0,
            equalifier: Box::new(ExactEqualifier::new()),
        }
    }
    pub fn execute_command(&mut self, cmd: &Command) -> Result<String, &str> {
        match cmd.cmd {
            CommandType::Set => {
                if !self.sources.contains_key(&cmd.source) {
                    self.sources.insert(
                        cmd.source.to_string(),
                        Source {
                            name: cmd.source.to_string(),
                            quality: self.default_source_quality,
                            strength: self.initial_source_strength,
                        },
                    );
                }
                if !self.questions.contains_key(&cmd.question) {
                    self.questions.insert(
                        cmd.question.to_string(),
                        Question {
                            name: cmd.question.to_string(),
                            correct_answers: Vec::new(),
                            confidence: 0.0,
                            weight: 0.0,
                            sources: Vec::new(),
                            answers: Vec::new(),
                        },
                    );
                }
                let answer = Answer::new(cmd.answer.clone(), cmd.source.clone());
                let mut question = self.questions.get_mut(&cmd.question).unwrap();

                // Remove effect of question
                {
                    let mut correct_answers: HashSet<u64> = HashSet::new();
                    for a in &question.correct_answers {
                        correct_answers.insert(a.hash);
                    }
                    for a in &question.answers {
                        let originally_correct_fac = if correct_answers.contains(&a.hash) {
                            1.
                        } else {
                            0.
                        };
                        let answer_source = self.sources.get_mut(&a.source).unwrap();
                        let new_quality = (answer_source.quality * answer_source.strength as f64
                            - question.weight * originally_correct_fac)
                            / (answer_source.strength as f64 - question.weight);
                        println!(
                            "(revert) Adjusting {}.quality {} -> {}",
                            answer_source.name, answer_source.quality, new_quality
                        );
                        println!(
                            "(revert) Adjusting {}.strength {} -> {}",
                            answer_source.name,
                            answer_source.strength,
                            answer_source.strength - question.weight
                        );
                        answer_source.strength -= question.weight;
                        answer_source.quality = new_quality;
                    }
                }

                question.answers.push(answer);

                let new_source = self.sources.get(&cmd.source).unwrap();

                // Modify sources to indicate whether or not they're correct or incorrect
                let clusters: Vec<Vec<usize>> =
                    compute_clusters(&question.answers, self.equalifier.as_ref()).unwrap();
                let mut cluster_confidences: Vec<f64> = vec![0.0; clusters.len()];

                // println!(
                //     "previously correct sources: {:?}",
                //     previously_correct_sources
                // );
                // println!("previously correct answers: {:?}", question.correct_answers);

                for (cluster_index, cluster_members) in clusters.iter().enumerate() {
                    let sources = &self.sources;
                    let incorrect_chance =
                        cluster_members.iter().fold(1.0_f64, |acc, &answer_index| {
                            let answer: &Answer = &question.answers[answer_index];
                            let member_source_quality: f64 = sources[&answer.source].quality;
                            // TODO apply function to lessen effect of guessing (e.g., 50% -> ~10%, 90% -> ~90%)
                            acc * (1.0 - member_source_quality)
                        });
                    cluster_confidences[cluster_index] = 1.0 - incorrect_chance;
                }

                println!("cluster confidences: {:?}", cluster_confidences);

                let correct_cluster: usize = argmaxf(&cluster_confidences);

                // TODO sort by best source first
                question.correct_answers = clusters[correct_cluster]
                    .iter()
                    .map(|answer_index| question.answers[*answer_index].clone())
                    .collect();
                println!(
                    "{}.confidence {} -> {}",
                    question.name, question.confidence, cluster_confidences[correct_cluster]
                );
                question.confidence = cluster_confidences[correct_cluster];
                let new_weight = if question.correct_answers.len() > 1 {
                    1.0
                } else {
                    0.0
                };
                println!(
                    "{}.weight {} -> {}",
                    question.name, question.weight, new_weight
                );
                question.weight = new_weight;

                // Add effect of question
                {
                    let mut correct_answers: HashSet<u64> = HashSet::new();
                    for a in &question.correct_answers {
                        correct_answers.insert(a.hash);
                    }
                    for a in &question.answers {
                        let originally_correct_fac = if correct_answers.contains(&a.hash) {
                            1.
                        } else {
                            -0.
                        };
                        let answer_source = self.sources.get_mut(&a.source).unwrap();
                        let new_quality = (answer_source.quality * answer_source.strength
                            + question.weight * originally_correct_fac)
                            / (answer_source.strength as f64 + question.weight);
                        println!(
                            "Adjusting {}.quality {} -> {}",
                            answer_source.name, answer_source.quality, new_quality
                        );
                        println!(
                            "Adjusting {}.strength {} -> {}",
                            answer_source.name,
                            answer_source.strength,
                            answer_source.strength + question.weight
                        );
                        answer_source.strength += question.weight;
                        answer_source.quality = new_quality;
                    }
                }

                return Ok(String::from(""));
            }
            CommandType::Get => {
                // TODO recompute question answer from sources if confidence is below a threshold
                let question: &Question = self.questions.get(&cmd.question).unwrap();
                let default_answer: Answer = Answer::new(String::from("None"), String::from(""));
                let correct_answer = question
                    .correct_answers
                    .first()
                    .or_else(|| Some(&default_answer))
                    .unwrap();
                Ok(format!("{} {}", question.confidence, correct_answer))
            }
        }
    }
}

#[test]
fn test_graph_1() {
    let commands: Vec<Command> = "\
    SET q1 a FROM s1
    SET q1 a FROM s2
    SET q1 a FROM s3
    SET q1 w FROM s4

    SET q2 b FROM s1
    SET q2 c FROM s2
    SET q2 b FROM s3
    SET q2 w FROM s4

    SET q3 d FROM s1
    SET q4 e FROM s2
    SET q5 f FROM s3
    SET q6 w FROM s4

    GET q1
    GET q2
    GET q3  
    GET q4  
    GET q5  
    GET q6  
    "
    .lines()
    .filter(|l| !l.trim().is_empty())
    .map(|l| Command::from(l.trim()))
    .collect();

    let mut g = Graph::new();

    let mut outputs: Vec<String> = Vec::new();

    for command in &commands {
        println!("\n{}", command);
        let output = g.execute_command(&command).unwrap();
        if output.len() > 1 {
            println!("> {}", output);
        }
        outputs.push(output);
    }

    assert_eq!(outputs[0], "");
    assert_eq!(outputs[1], "");
    assert_eq!(outputs[2], "");
    assert_eq!(outputs[3], "");
    assert_eq!(outputs[4], "");
    // assert_eq!(outputs[5], "q1 0 None");

    // println!("{:?}", outputs);
}
