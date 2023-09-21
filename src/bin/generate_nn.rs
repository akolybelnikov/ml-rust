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
use serde::Serialize;

#[derive(StructOpt)]
struct Options {
    #[structopt(short = "c", long = "config-file", parse(from_os_str))]
    config_file_path: std::path::PathBuf,
}

#[derive(serde::Deserialize)]
struct Config {
    centroids: [f64; 4],
    noise: f64,
    samples_per_centroid: usize,
}

#[derive(Debug, Serialize)]
struct Sample {
    height: f64,
    length: f64,
    category_id: usize,
}

fn generate_data(centroids: &Matrix<f64>, points_per_centroid: usize, noise: f64) -> Vec<Sample> {
    assert!(centroids.cols() > 0, "Centroids must be non-empty");
    assert!(centroids.rows() > 0, "Centroids must be non-empty");
    assert!(noise >= 0f64, "Noise must be non-negative");

    let mut samples = Vec::with_capacity(points_per_centroid);

    let mut rng = thread_rng();

    let normal_rv = Normal::new(0f64, noise).unwrap();

    for _ in 0..points_per_centroid {
        for (centroid_id, centroid) in centroids.iter_rows().enumerate() {
            let mut point = Vec::with_capacity(centroids.cols());
            for feature in centroid.iter() {
                point.push(feature + normal_rv.sample(&mut rng));
            }
            samples.push(Sample {
                height: point[0],
                length: point[1],
                category_id: centroid_id,
            });
        }
    }

    samples
}

fn main() -> Result<(), io::Error> {
    let options = Options::from_args();
    let toml_config_str = read_to_string(options.config_file_path)?;
    let config_result: Result<Config, toml::de::Error> = toml::from_str(&toml_config_str);
    let config = match config_result {
        Ok(config) => config,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };
    let centroids = Matrix::new(2, 2, config.centroids.to_vec());
    let samples = generate_data(&centroids, config.samples_per_centroid, config.noise);
    let mut writer = csv::Writer::from_writer(io::stdout());
    for sample in samples.iter() {
        writer.serialize(sample)?;
    }
    writer.flush()?;
    Ok(())
}
