pub use crate::command::Answer;

mod exact_equalifier;
mod numeric_equalifier;
mod numeric_vec_equalifier;

pub use self::exact_equalifier::ExactEqualifier;
pub use self::numeric_equalifier::NumericEqualifier;
pub use self::numeric_vec_equalifier::{NumericVecEqualifier, VecDistAlgo};

pub trait Equalifier {
    fn is_valid_answer(&self, a: &Answer) -> bool;
    fn get_distance(&self, a: &Answer, b: &Answer) -> f64;
}
