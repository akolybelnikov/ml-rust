extern crate rand;
extern crate rusty_machine;
use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::Normal;
use rusty_machine::linalg::{BaseMatrix, Matrix};
use std::io;
use std::vec::Vec;

fn generate_data(centroids: &Matrix<f64>, points_per_centroid: usize, noise: f64) -> Matrix<f64> {
    assert!(centroids.cols() > 0, "Centroids must be non-empty");
    assert!(centroids.rows() > 0, "Centroids must be non-empty");
    assert!(noise >= 0f64, "Noise must be non-negative");

    let mut raw_cluster_data =
        Vec::with_capacity(centroids.cols() * points_per_centroid * centroids.rows());

    let mut rng = thread_rng();

    let normal_rv = Normal::new(0f64, noise).unwrap();

    for _ in 0..points_per_centroid {
        for centroid in centroids.iter_rows() {
            let mut point = Vec::with_capacity(centroids.cols());
            for feature in centroid.iter() {
                point.push(feature + normal_rv.sample(&mut rng));
            }
            raw_cluster_data.extend(point);
        }
    }

    Matrix::new(
        centroids.rows() * points_per_centroid,
        centroids.cols(),
        raw_cluster_data,
    )
}

fn main() {}