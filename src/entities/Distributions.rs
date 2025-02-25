

use serde::{Deserialize, Serialize};


pub trait Distribution<'a>: 'a {
    fn new(params: Vec<&'a [f64]>) -> Self;
    fn to_json(&self) -> String;
}


/// Multidimensional normal distribution specified by means and a covariance matrix.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Default)]
pub struct FullNormal<'a> {
    pub mu: &'a [f64],
    pub cov: &'a [f64],
}

impl<'a> Distribution<'a> for FullNormal<'a> {
    fn new(
        params: Vec<&'a [f64]>
    ) -> Self {
        Self {
            mu: params[0],
            cov: params[1],
        }
    }

    /// Convert to JSON, notice change in type to `PackedFullNormal`
    fn to_json(
        &self
    ) -> String {
        return PackedFullNormal {
            mu: self.mu.to_vec(),
            cov: self.cov.to_vec(),
            type_: "IncrementalInference.PackedFullNormal".to_string(),
        }.to_json();
    }
}


/// Multidimensional normal distribution specified by means and a covariance matrix.
#[allow(non_snake_case)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PackedFullNormal {
    pub mu: Vec<f64>,
    pub cov: Vec<f64>,
    pub type_: String,
}


impl PackedFullNormal {
    pub fn to_json(
        &self
    ) -> String {
        serde_json::to_string(self).unwrap().to_string()
    }
}

