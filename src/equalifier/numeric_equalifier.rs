use crate::equalifier::{Answer, Equalifier};
use num::clamp;

pub struct NumericEqualifier {
    max_distance: f64,
}

impl NumericEqualifier {
    pub fn new(max_distance: f64) -> Self {
        NumericEqualifier { max_distance }
    }
}

impl Equalifier for NumericEqualifier {
    fn get_distance(&self, a: &Answer, b: &Answer) -> f64 {
        let af = a.content.parse::<f64>().unwrap();
        let bf = b.content.parse::<f64>().unwrap();
        clamp((af - bf).abs() / self.max_distance, 0.0, 1.0)
    }
    fn is_valid_answer(&self, a: &Answer) -> bool {
        a.content.parse::<f64>().is_ok()
    }
}

#[test]
fn numeric_distance_test() {
    let nd = NumericEqualifier::new(10.0);
    let a = Answer::new(String::from("2"), String::from("s1"));
    let b = Answer::new(String::from("8.56"), String::from("s2"));
    assert_eq!(nd.get_distance(&a, &b), (8.56 - 2.0) / 10.0);
}
