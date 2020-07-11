use crate::command::{Command, CommandType, Answer};
use crate::equalifier::{ExactEqualifier, Equalifier};
// use crate::dbscan::cluster;

pub fn equal_distance_fn(a: &Answer, b: &Answer) -> f64 {
    if a.content == b.content {
        return 0.0;
    } else {
        return 1.0;
    }
}

pub fn compute_clusters(answers: &Vec<Answer>, equalifier: &Equalifier) -> Result<Vec<Vec<Answer>>, String> {
    // Compute answer distances
    let N = answers.len();
    let mut distances: Vec<Vec<f64>> = (0..N).map(|_| { vec![0.0; N] }).collect();
    for i in 0..N {
        for u in i..N {
            if i == u {
                distances[i][u] = 0.0;
            } else {
                let iudist: f64 = equalifier.get_distance(&answers[i], &answers[u]);
                distances[i][u] = iudist;
                distances[u][i] = iudist;
            }
        }
    }
    println!("distances:\n:{:?}", distances);
    Err(String::from("Not Finished"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_clusters() {
        compute_clusters(
            &vec![
                Answer::new(String::from("a"), String::from("s1")),
                Answer::new(String::from("b"), String::from("s2"))
            ],
            &ExactEqualifier{}
        );
    }
}