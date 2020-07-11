use crate::command::{Answer};
use num::{clamp};
use assert_approx_eq::assert_approx_eq;

pub trait Equalifier {
    fn is_valid_answer(&self, a: &Answer) -> bool;
    fn get_distance(&self, a: &Answer, b: &Answer) -> f64;
}

pub struct OnlyEqual {}

impl Equalifier for OnlyEqual {
    fn is_valid_answer(&self, a: &Answer) -> bool { true }
    fn get_distance(&self, a: &Answer, b: &Answer) -> f64 {
        return if a.content == b.content { 0.0 } else { 1.0 }
    }
}

pub struct NumericDistance {
    max_distance: f64
}


impl NumericDistance {
    fn new(max_distance: f64) -> Self {
        NumericDistance { max_distance }
    }
}

impl Equalifier for NumericDistance {
    fn get_distance(&self, a: &Answer, b: &Answer) -> f64 {
        let af = a.content.parse::<f64>().unwrap();
        let bf = b.content.parse::<f64>().unwrap();
        clamp(((af - bf).abs() / self.max_distance), 0.0, 1.0)
    }
    fn is_valid_answer(&self, a: &Answer) -> bool {
        a.content.parse::<f64>().is_ok()
    }
}

#[test]
fn numeric_distance_test() {
    let nd = NumericDistance::new(10.0);
    let a = Answer::new(String::from("2"), String::from("s1"));
    let b = Answer::new(String::from("8.56"), String::from("s2"));
    assert_eq!(
        nd.get_distance(&a, &b),
        (8.56 - 2.0) / 10.0
    );
}

pub enum vec_dist_algo {
    L2,
    L1
}

pub struct NumericVectorDistance {
    max_distance: f64,
    dist_fn: vec_dist_algo
}

impl NumericVectorDistance {
    fn new(max_distance: f64, dist_fn: vec_dist_algo) -> Self {
        NumericVectorDistance { max_distance, dist_fn }
    }
}

fn split_to_f64_vec(a: &Answer, delimeter: &str) -> Vec<f64> {
    a.content.split(delimeter).map(|e| {
        e.parse::<f64>().unwrap()
    }).collect()
}

impl Equalifier for NumericVectorDistance {
    fn get_distance(&self, a: &Answer, b: &Answer) -> f64 {
        let av:Vec<f64> = split_to_f64_vec(a, &",");
        let bv:Vec<f64> = split_to_f64_vec(b, &",");
        if av.len() != bv.len() { return 1.0 }; // invalid dimensions, maximum error
        match self.dist_fn {
            vec_dist_algo::L2 => {
                (0..av.len())
                    .map(|i| { (av[i] - bv[i]).powi(2) })
                    .sum::<f64>()
                    .sqrt()
            }
            vec_dist_algo::L1 => {
                (0..av.len())
                    .map(|i| { (av[i] - bv[i]).abs() })
                    .sum()
            }
        }
    }
    fn is_valid_answer(&self, a: &Answer) -> bool {
        a.content.parse::<f64>().is_ok()
    }
}

#[test]
fn numeric_vector_distance_test_l1() {
    let nd = NumericVectorDistance::new(1.0, vec_dist_algo::L1);
    let a = Answer::new(String::from("1.0,2.0"), String::from("s1"));
    let b = Answer::new(String::from("1.1,2.1"), String::from("s2"));
    assert_approx_eq!(
        nd.get_distance(&a, &b),
        0.2
    );
}

#[test]
fn numeric_vector_distance_test_l2() {
    let nd = NumericVectorDistance::new(1.0, vec_dist_algo::L2);
    let a = Answer::new(String::from("1.0,2.0"), String::from("s1"));
    let b = Answer::new(String::from("1.1,2.1"), String::from("s2"));
    assert_approx_eq!(
        nd.get_distance(&a, &b),
        (0.02_f64).sqrt()
    );
}