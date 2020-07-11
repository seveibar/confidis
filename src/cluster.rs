use crate::command::Answer;
use crate::equalifier::{Equalifier, ExactEqualifier, NumericEqualifier};

pub fn equal_distance_fn(a: &Answer, b: &Answer) -> f64 {
    if a.content == b.content {
        return 0.0;
    } else {
        return 1.0;
    }
}

pub fn compute_clusters(
    answers: &Vec<Answer>,
    equalifier: &Equalifier,
) -> Result<Vec<Vec<usize>>, String> {
    // Compute answer distances
    let N = answers.len();
    // TODO (not important, probably) the distance function doesn't need to have duplicates since all distances
    // are assumed to be symmetric.
    let mut distances: Vec<Vec<f64>> = (0..N).map(|_| vec![0.0; N]).collect();
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

    let model = dbscan::Model::new(1.0, 1);
    // TODO euclidean distance is used by default, is that the best way to cluster?
    // model.set_distance_fn(...)

    let dbscan_output = model.run(&distances);
    // println!("distances:\n:{:?}", distances);
    // println!("dbscan_output: {:?}", dbscan_output);

    let mut number_of_clusters: usize = 0;
    for location in &dbscan_output {
        match location {
            dbscan::Classification::Core(cluster) | dbscan::Classification::Edge(cluster) => {
                if cluster + 1 > number_of_clusters {
                    number_of_clusters = cluster + 1;
                }
            }
            dbscan::Classification::Noise => {}
        }
    }

    let mut clustered_answers: Vec<Vec<usize>> = vec![Vec::new(); number_of_clusters];

    for (i, location) in (&dbscan_output).iter().enumerate() {
        match location {
            dbscan::Classification::Core(cluster) | dbscan::Classification::Edge(cluster) => {
                clustered_answers[*cluster].push(i);
            }
            dbscan::Classification::Noise => {}
        }
    }

    // println!("number of clusters: {}", number_of_clusters);

    Ok(clustered_answers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_clusters_exact() {
        assert_eq!(
            compute_clusters(
                &vec![
                    Answer::new(String::from("a"), String::from("s1")),
                    Answer::new(String::from("b"), String::from("s2")),
                ],
                &ExactEqualifier {},
            )
            .unwrap(),
            vec![vec![0], vec![1]],
        );
    }

    #[test]
    fn test_compute_clusters_nums() {
        assert_eq!(
            compute_clusters(
                &vec![
                    Answer::new(String::from("0.2"), String::from("s1")),
                    Answer::new(String::from("0.5"), String::from("s2")),
                    Answer::new(String::from("2.4"), String::from("s3")),
                ],
                &NumericEqualifier::new(1.0),
            )
            .unwrap(),
            vec![vec![0, 1], vec![2]],
        );
    }
}
