use crate::equalifier::{Answer, Equalifier};
use assert_approx_eq::assert_approx_eq;
use num::clamp;

pub enum VecDistAlgo {
    L2Norm,
    L1Norm,
    PercentNotEqual,
}

pub struct NumericVecEqualifier {
    allowed_difference: f64,
    vec_length: usize,
    diff_fn: VecDistAlgo,
}

impl NumericVecEqualifier {
    pub fn new(allowed_difference: f64, diff_fn: VecDistAlgo, vec_length: usize) -> Self {
        NumericVecEqualifier {
            allowed_difference,
            diff_fn,
            vec_length,
        }
    }
}

fn split_to_f64_vec(a: &Answer, delimeter: &str) -> Vec<f64> {
    a.content
        .split(delimeter)
        .map(|e| e.parse::<f64>().unwrap())
        .collect()
}

impl Equalifier for NumericVecEqualifier {
    fn get_distance(&self, a: &Answer, b: &Answer) -> f64 {
        let av: Vec<f64> = split_to_f64_vec(a, &",");
        let bv: Vec<f64> = split_to_f64_vec(b, &",");
        if av.len() != bv.len() {
            return 1.0;
        }; // invalid dimensions, maximum error
        let normalize = |x| clamp(x / self.allowed_difference, 0.0, 1.0);
        match self.diff_fn {
            VecDistAlgo::L2Norm => normalize(
                (0..av.len())
                    .map(|i| (av[i] - bv[i]).powi(2))
                    .sum::<f64>()
                    .sqrt(),
            ),
            VecDistAlgo::L1Norm => normalize((0..av.len()).map(|i| (av[i] - bv[i]).abs()).sum()),
            VecDistAlgo::PercentNotEqual => normalize(
                (0..av.len()).filter(|&i| av[i] != bv[i]).count() as f64 / (av.len() as f64),
            ),
        }
    }
    fn is_valid_answer(&self, a: &Answer) -> bool {
        let av: Vec<f64> = split_to_f64_vec(a, &",");
        return av.len() == self.vec_length;
    }
}

#[test]
fn numeric_vector_distance_test_l1() {
    let nd = NumericVecEqualifier::new(1.0, VecDistAlgo::L1Norm, 2);
    let a = Answer::new(String::from("1.0,2.0"), String::from("s1"));
    let b = Answer::new(String::from("1.1,2.1"), String::from("s2"));
    assert_approx_eq!(nd.get_distance(&a, &b), 0.2);
}

#[test]
fn numeric_vector_distance_test_l2() {
    let nd = NumericVecEqualifier::new(1.0, VecDistAlgo::L2Norm, 2);
    let a = Answer::new(String::from("1.0,2.0"), String::from("s1"));
    let b = Answer::new(String::from("1.1,2.1"), String::from("s2"));
    assert_approx_eq!(nd.get_distance(&a, &b), (0.02_f64).sqrt());
}

#[test]
fn numeric_vector_distance_test_percent_not_equal() {
    let nd = NumericVecEqualifier::new(0.25, VecDistAlgo::PercentNotEqual, 10);
    let a = Answer::new(String::from("1,2,3,4,5,6,7,8,9,10"), String::from("s1"));
    let b = Answer::new(String::from("1,1,3,4,5,6,7,8,9,10"), String::from("s2"));
    assert_approx_eq!(nd.get_distance(&a, &b), 0.1 / 0.25);
}
