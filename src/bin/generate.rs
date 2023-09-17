extern crate rand;
extern crate rusty_machine;

use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::Normal;
use rusty_machine::linalg::{BaseMatrix, Matrix};
use std::fs::read_to_string;
use std::io;
use std::vec::Vec;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(short = "c", long = "config-file", parse(from_os_str))]
    config_file_path: std::path::PathBuf,
}

#[derive(serde::Deserialize)]
struct Config {
    centroids: [f64; 6],
    noise: f64,
    samples_per_centroid: usize,
}

fn generate_data(centroids: &Matrix<f64>, samples_per_centroid: usize, noise: f64) -> Matrix<f64> {
    assert!(centroids.cols() > 0, "Centroids must be non-empty");
    assert!(centroids.rows() > 0, "Centroids must be non-empty");
    assert!(noise >= 0f64, "Noise must be non-negative");

    let mut raw_cluster_data =
        Vec::with_capacity(centroids.cols() * samples_per_centroid * centroids.rows());

    let mut rng = thread_rng();

    let normal_rv = Normal::new(0f64, noise).unwrap();

    for _ in 0..samples_per_centroid {
        for centroid in centroids.iter_rows() {
            let mut point = Vec::with_capacity(centroids.cols());
            for feature in centroid.iter() {
                point.push(feature + normal_rv.sample(&mut rng));
            }
            raw_cluster_data.extend(point);
        }
    }

    Matrix::new(
        centroids.rows() * samples_per_centroid,
        centroids.cols(),
        raw_cluster_data,
    )
}

fn main() -> Result<(), std::io::Error> {
    let options = Options::from_args();
    let toml_config_str = read_to_string(options.config_file_path)?;
    let config_result: Result<Config, toml::de::Error> = toml::from_str(&toml_config_str);
    let config = match config_result {
        Ok(config) => config,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };
    let centroids = Matrix::new(3, 2, config.centroids.to_vec());
    let samples = generate_data(&centroids, config.samples_per_centroid, config.noise);
    let mut writer = csv::Writer::from_writer(io::stdout());
    writer.write_record(&["height", "length"])?;
    for sample in samples.iter_rows() {
        writer.serialize(sample)?;
    }
    writer.flush()?;
    Ok(())
}
