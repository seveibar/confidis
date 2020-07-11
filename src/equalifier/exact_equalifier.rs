use crate::equalifier::Equalifier;
use crate::command::{Answer};

pub struct ExactEqualifier {}

impl Equalifier for ExactEqualifier {
    fn is_valid_answer(&self, _a: &Answer) -> bool { true }
    fn get_distance(&self, a: &Answer, b: &Answer) -> f64 {
        return if a.content == b.content { 0.0 } else { 1.0 }
    }
}