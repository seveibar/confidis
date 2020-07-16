extern crate wasm_bindgen;
use crate::cluster::compute_clusters;
use crate::command::{Answer, Command, CommandResponse, CommandType};
use crate::equalifier::{
    Equalifier, ExactEqualifier, NumericEqualifier, NumericVecEqualifier, VecDistAlgo,
};
use log::info;
use std::collections::HashMap;
use std::collections::HashSet;
use std::result::Result;

use wasm_bindgen::prelude::*;

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

impl Default for Question {
    fn default() -> Self {
        Question {
            name: String::new(),
            correct_answers: Vec::new(),
            confidence: 0.0,
            weight: 0.0,
            sources: Vec::new(),
            answers: Vec::new(),
        }
    }
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
    default_source_quality: f64,
    initial_source_strength: f64,
    maximum_strength: f64,
    log_weight_factor: f64,
    quality_of_believed_sources: f64,
    equalifier: Box<dyn Equalifier>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            // All sources in system
            sources: HashMap::new(),

            // All questions in graph
            questions: HashMap::new(),

            // Default probability that a source will be correct
            default_source_quality: 0.5,

            // Starting strength of a source, if this is low (1.0) the initial quality will be changed
            // easily by new data. If higher, it's easier to resist adversaries with the "start good, turn bad"
            // attack
            initial_source_strength: 1.0,

            // Maximum strength of a source, impacts how effected they are by more recent
            // correct/incorrect answers
            maximum_strength: 100.0,

            // weight_of_question = -1. * log_{log_weight_factor}(1 - confidence)
            // 10.0 means that 90% confidence has a weight of 1. 99% confidence has a weight of 2. 99.9% has a weight of 3.
            log_weight_factor: 10.0,

            // quality of believed sources
            quality_of_believed_sources: 0.999,

            // The equality/similarity system used to compare answers
            equalifier: Box::new(ExactEqualifier::new()),
        }
    }

    // Modify connected sources to indicate whether or not they're correct or incorrect
    fn add_question_effect(&mut self, question_name: &str) {
        let question = self.questions.get_mut(question_name).unwrap();
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
            let new_quality = (answer_source.quality * answer_source.strength
                + question.weight * originally_correct_fac)
                / (answer_source.strength as f64 + question.weight);
            info!(
                "Adjusting {}.quality  {:.2} -> {:.2}",
                answer_source.name, answer_source.quality, new_quality
            );
            info!(
                "Adjusting {}.strength {:.2} -> {:.2}",
                answer_source.name,
                answer_source.strength,
                answer_source.strength + question.weight
            );
            answer_source.strength =
                (answer_source.strength + question.weight).min(self.maximum_strength);
            answer_source.quality = new_quality;
        }
    }

    // Revert the effect of this question on any connected sources
    fn remove_question_effect(&mut self, question_name: &str) {
        let question = self.questions.get_mut(question_name).unwrap();
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
            info!(
                "(revert) Adjusting {}.quality  {:.2} -> {:.2}",
                answer_source.name, answer_source.quality, new_quality
            );
            info!(
                "(revert) Adjusting {}.strength {:.2} -> {:.2}",
                answer_source.name,
                answer_source.strength,
                answer_source.strength - question.weight
            );
            answer_source.strength -= question.weight;
            answer_source.quality = new_quality;
        }
    }

    fn compute_question_answers(&mut self, question_name: &str) -> Result<(), String> {
        let mut question = self.questions.get_mut(question_name).unwrap();
        let clusters: Vec<Vec<usize>> =
            compute_clusters(&question.answers, self.equalifier.as_ref()).unwrap();
        let mut cluster_confidences: Vec<f64> = vec![0.0; clusters.len()];

        for (cluster_index, cluster_members) in clusters.iter().enumerate() {
            let sources = &self.sources;
            let incorrect_chance = cluster_members.iter().fold(1.0_f64, |acc, &answer_index| {
                let answer: &Answer = &question.answers[answer_index];
                let member_source_quality: f64 = sources[&answer.source].quality;
                // TODO apply function to lessen effect of guessing (e.g., 50% -> ~10%, 90% -> ~90%)
                acc * (1.0 - member_source_quality)
            });
            cluster_confidences[cluster_index] = 1.0 - incorrect_chance;
        }

        info!("cluster confidences: {:?}", cluster_confidences);

        let correct_cluster: usize = argmaxf(&cluster_confidences);

        // TODO sort by best source first
        question.correct_answers = clusters[correct_cluster]
            .iter()
            .map(|answer_index| question.answers[*answer_index].clone())
            .collect();
        info!(
            "Adjusting {}.confidence {:.2} -> {:.2}",
            question.name, question.confidence, cluster_confidences[correct_cluster]
        );
        question.confidence = cluster_confidences[correct_cluster];
        let new_weight = if question.correct_answers.len() > 1 {
            // 1.0
            -1.0 * (1.0 - question.confidence).log(self.log_weight_factor)
        } else {
            0.0
        };
        info!(
            "Adjusting {}.weight     {:.2} -> {:.2}",
            question.name, question.weight, new_weight
        );
        question.weight = new_weight;
        Ok(())
    }

    pub fn execute_command(&mut self, cmd: &Command) -> Result<CommandResponse, String> {
        match cmd.cmd {
            CommandType::Set => {
                let source_name = cmd.source.as_ref().unwrap();
                let question_name = cmd.question.as_ref().unwrap();

                if !self.sources.contains_key(source_name) {
                    self.sources.insert(
                        source_name.to_string(),
                        Source {
                            name: source_name.to_string(),
                            quality: self.default_source_quality,
                            strength: self.initial_source_strength,
                        },
                    );
                }
                if !self.questions.contains_key(question_name) {
                    self.questions.insert(
                        question_name.to_string(),
                        Question {
                            name: question_name.to_string(),
                            ..Default::default()
                        },
                    );
                }
                let answer = Answer::new(cmd.answer.as_ref().unwrap().clone(), source_name.clone());

                self.remove_question_effect(question_name);
                {
                    let question = self.questions.get_mut(question_name).unwrap();
                    question.answers.push(answer);
                }
                self.compute_question_answers(question_name)
                    .expect("error computing question answer");
                self.add_question_effect(question_name);

                Ok(CommandResponse {
                    cmd: CommandType::Set,
                    ..Default::default()
                })
            }
            CommandType::GetAnswer => {
                let question_name = cmd.question.as_ref().unwrap();

                self.remove_question_effect(question_name);
                self.compute_question_answers(question_name)
                    .expect("error computing question answer");
                self.add_question_effect(question_name);

                let question: &Question = self.questions.get(question_name).unwrap();
                let default_answer: Answer = Answer::new(String::from("None"), String::from(""));
                let correct_answer = question
                    .correct_answers
                    .first()
                    .or_else(|| Some(&default_answer))
                    .unwrap();
                Ok(CommandResponse {
                    cmd: CommandType::GetAnswer,
                    confidence: Some(question.confidence),
                    answer: Some(correct_answer.content.clone()),
                })
            }
            CommandType::Configure => {
                let config_key = cmd.config_key.as_ref().unwrap();
                let config_val = cmd.config_val.as_ref().unwrap();
                let params: HashMap<&str, &str> = config_val
                    .split_whitespace()
                    .filter(|&s| s.contains("="))
                    .collect::<Vec<&str>>()
                    .iter()
                    .fold(HashMap::new(), |mut acc, s| {
                        let mut components = s.split("=");
                        acc.insert(components.next().unwrap(), components.next().unwrap());
                        acc
                    });

                match config_key.as_str() {
                    "comparison_method" => match config_val.split_whitespace().next().unwrap() {
                        "exact" => self.equalifier = Box::new(ExactEqualifier {}),
                        "numeric" => {
                            let max_distance = params
                                .get("max_distance")
                                .and_then(|d| d.parse::<f64>().ok());

                            if !max_distance.is_some() {
                                return Err("max_distance must be specified".into());
                            }

                            self.equalifier = Box::new(NumericEqualifier {
                                max_distance: max_distance.unwrap(),
                            })
                        }
                        "numeric_vec" => {
                            let allowed_difference = params
                                .get("allowed_difference")
                                .and_then(|s| s.parse::<f64>().ok());

                            let vec_length = params
                                .get("vec_length")
                                .and_then(|s| s.parse::<usize>().ok());

                            let diff_fn: Option<VecDistAlgo> =
                                params.get("diff_fn").and_then(|s| VecDistAlgo::from(s));

                            if !allowed_difference.is_some() {
                                return Err("allowed_difference must be specified (try 1.0)".into());
                            }
                            if !vec_length.is_some() {
                                return Err(
                                    "vec_length must be specified (vector lengths must be fixed)"
                                        .into(),
                                );
                            }
                            if !diff_fn.is_some() {
                                return Err(
                                    "diff_fn must be specified (l1, l2, percent_not_equal, iou)"
                                        .into(),
                                );
                            }

                            self.equalifier = Box::new(NumericVecEqualifier {
                                allowed_difference: allowed_difference.unwrap(),
                                vec_length: vec_length.unwrap(),
                                diff_fn: diff_fn.unwrap(),
                            })
                        }
                        &_ => {
                            return Err(format!("unknown comparison method \"{}\". Try exact, numeric, or numeric_vec", config_key));
                        }
                    },
                    "default_source_quality" => {
                        if let Ok(v) = (&config_val).parse() {
                            self.default_source_quality = v;
                        }
                    }
                    "log_weight_factor" => {
                        if let Ok(v) = (&config_val).parse() {
                            self.log_weight_factor = v;
                        }
                    }
                    "initial_source_strength" => {
                        if let Ok(v) = (&config_val).parse() {
                            self.initial_source_strength = v;
                        }
                    }
                    "maximum_strength" => {
                        if let Ok(v) = (&config_val).parse() {
                            self.maximum_strength = v;
                        }
                    }
                    &_ => {
                        return Err(format!("Unknown configuration key: \"{}\"", config_key));
                    }
                }

                Ok(CommandResponse {
                    cmd: CommandType::Configure,
                    ..Default::default()
                })
            }
            _ => Err("Not implemented or invalid command".into()),
        }
    }
}

#[test]
fn test_graph_1() {
    pretty_env_logger::init();
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

    GET ANSWER TO q1
    GET ANSWER TO q2
    GET ANSWER TO q3  
    GET ANSWER TO q4  
    GET ANSWER TO q5  
    GET ANSWER TO q6  
    "
    .lines()
    .filter(|l| !l.trim().is_empty())
    .map(|l| Command::from(l.trim()))
    .filter_map(|x| x.ok())
    .collect();

    let mut g = Graph::new();

    let mut outputs: Vec<String> = Vec::new();

    for command in &commands {
        info!("\n{}", command);
        let output = g.execute_command(&command).unwrap();
        if output.cmd == CommandType::GetAnswer {
            info!("> {}", output);
            outputs.push(format!("> {}", &output));
        }
    }

    assert_eq!(
        outputs.join("\n"),
        "\
> a (95.885%)
> b (95.607%)
> d (86.641%)
> e (50.379%)
> f (86.641%)
> w (13.359%)"
    );
}
