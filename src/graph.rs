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
    strength: i64,
}

#[derive(Debug)]
pub struct Question {
    name: QuestionId,
    correct_answers: Vec<Answer>,
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
    initial_source_strength: i64,
    strength_maximum: i64,
    minimum_influence_votes: i64,
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
            minimum_influence_votes: 2,
            default_source_quality: 0.5,
            initial_source_strength: 1,
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
                            sources: Vec::new(),
                            answers: Vec::new(),
                        },
                    );
                }
                let answer = Answer::new(cmd.answer.clone(), cmd.source.clone());
                let mut question = self.questions.get_mut(&cmd.question).unwrap();
                let influential_question = question.confidence >= self.influence_source_confidence
                    && question.answers.len() >= self.minimum_influence_votes as usize;

                // Update source quality if answer has high enough confidence
                if influential_question {
                    let mut source = self.sources.get_mut(&cmd.source).unwrap();
                    let correct_answer = &question.correct_answers[0];
                    // update source quality
                    // TODO put distance function here & clustering
                    let answer_distance = self.equalifier.get_distance(&answer, &correct_answer);
                    if answer_distance < 1.0 {
                        println!(
                            "CORRECT: source ({}) matches answer for question ({})",
                            cmd.source, cmd.question
                        );
                        let new_source_quality = (source.quality * source.strength as f64 + 1.0)
                            / (source.strength as f64 + 1.0);
                        println!(
                            "Adjusting {}.quality {} -> {}",
                            source.name, source.quality, new_source_quality
                        );
                        source.quality = new_source_quality;
                        question.correct_answers.push(answer.clone());
                    } else {
                        println!(
                            "WRONG: source ({}) doesn't match answer for question ({})",
                            cmd.source, cmd.question
                        );
                        let new_source_quality = (source.quality * source.strength as f64)
                            / (source.strength as f64 + 1.0);
                        println!(
                            "Adjusting {}.quality {} -> {}",
                            source.name, source.quality, new_source_quality
                        );
                        source.quality = new_source_quality;
                    }
                    if source.strength < self.strength_maximum {
                        println!(
                            "Adjusting {}.strength {} -> {}",
                            source.name,
                            source.strength,
                            source.strength + 1
                        );
                        source.strength += 1;
                    }
                }

                question.answers.push(answer);

                let new_source = self.sources.get(&cmd.source).unwrap();

                // Modify sources to indicate whether or not they're correct or incorrect
                if new_source.strength >= self.influence_question_strength {
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
                    let previously_influential_question = influential_question;
                    let influential_question = question.confidence
                        >= self.influence_source_confidence
                        && question.answers.len() >= self.minimum_influence_votes as usize;
                    let mut previously_correct_sources: HashSet<String> = HashSet::new();
                    if previously_influential_question {
                        for a in &question.correct_answers {
                            previously_correct_sources.insert(a.source.clone());
                        }
                    }

                    // TODO sort by best source first
                    question.correct_answers = clusters[correct_cluster]
                        .iter()
                        .map(|answer_index| question.answers[*answer_index].clone())
                        .collect();
                    question.confidence = cluster_confidences[correct_cluster];

                    let mut currently_correct_sources: HashSet<String> = HashSet::new();
                    for answer_index in &clusters[correct_cluster] {
                        currently_correct_sources
                            .insert(question.answers[*answer_index].source.clone());
                    }

                    // Modify sources according to their previous and new statuses
                    if influential_question {
                        for answer in &question.answers {
                            let prev_correct = previously_correct_sources.contains(&answer.source);
                            let curr_correct = currently_correct_sources.contains(&answer.source);
                            if prev_correct != curr_correct {
                                let answer_source = self.sources.get_mut(&answer.source).unwrap();

                                if curr_correct && previously_influential_question {
                                    // Increase quality (without strength adjustment)
                                    let new_quality = (answer_source.quality
                                        * (answer_source.strength as f64)
                                        + 2.0)
                                        / (answer_source.strength as f64);
                                    println!(
                                        "Adjusting {}.quality {} -> {}",
                                        answer_source.name, answer_source.quality, new_quality
                                    );
                                    answer_source.quality = new_quality;
                                } else if curr_correct && !previously_influential_question {
                                    // Increase quality (with strength adjustment)
                                    let new_quality = (answer_source.quality
                                        * (answer_source.strength as f64)
                                        + 1.0)
                                        / (answer_source.strength as f64 + 1.0);
                                    println!(
                                        "Adjusting {}.quality {} -> {}",
                                        answer_source.name, answer_source.quality, new_quality
                                    );
                                    println!(
                                        "Adjusting {}.strength {} -> {}",
                                        answer_source.name,
                                        answer_source.strength,
                                        answer_source.strength + 1
                                    );
                                    if answer_source.strength < self.strength_maximum {
                                        answer_source.strength += 1;
                                    }
                                    answer_source.quality = new_quality;
                                } else if previously_influential_question {
                                    // Decrease quality (w/o strength adjustment)
                                    let new_quality = (answer_source.quality
                                        * answer_source.strength as f64
                                        - 2.0)
                                        / (answer_source.strength as f64);
                                    println!(
                                        "Adjusting {}.quality {} -> {}",
                                        answer_source.name, answer_source.quality, new_quality
                                    );
                                    answer_source.quality = new_quality;
                                } else {
                                    // Decrease quality (w/ strength adjustment)
                                    let new_quality = (answer_source.quality
                                        * answer_source.strength as f64
                                        - 1.0)
                                        / (answer_source.strength as f64 + 1.0);
                                    println!(
                                        "Adjusting {}.quality {} -> {}",
                                        answer_source.name, answer_source.quality, new_quality
                                    );
                                    println!(
                                        "Adjusting {}.strength {} -> {}",
                                        answer_source.name,
                                        answer_source.strength,
                                        answer_source.strength + 1
                                    );
                                    if answer_source.strength < self.strength_maximum {
                                        answer_source.strength += 1;
                                    }
                                    answer_source.quality = new_quality;
                                }
                            }
                        }
                    }
                }

                return Ok(String::from(""));
            }
            CommandType::Get => {
                let question: &Question = self.questions.get(&cmd.question).unwrap();
                let default_answer: Answer = Answer::new(String::from("None"), String::from(""));
                let correct_answer = question
                    .correct_answers
                    .first()
                    .or_else(|| Some(&default_answer))
                    .unwrap();
                Ok(format!("{} {}", question.confidence, correct_answer))
                // return Ok(String::from("Hi"))
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
