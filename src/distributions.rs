//! Defines additional distributions to those of statrs
use rand::Rng;
use statrs::{
    distribution::{Continuous, Uniform},
    StatsError,
};

use crate::traits::Dist;

/// n-dimensional continuous uniform distribution
// TODO: initialize and store Uniform in the struct?
pub struct NDUniform {
    n: usize,
}

impl NDUniform {
    pub fn new(n: usize) -> statrs::Result<Self> {
        if n == 0 {
            return Err(StatsError::BadParams);
        }
        Ok(Self { n })
    }
}

impl ::rand::distributions::Distribution<Vec<f64>> for NDUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec<f64> {
        let d = Uniform::new(0., 1.).unwrap();
        let z = d.sample_iter(rng).by_ref().take(self.n).collect();
        z
    }
}

impl Dist<Vec<f64>> for NDUniform {
    fn density(&self) -> Box<dyn Fn(Vec<f64>) -> f64 + '_> {
        let d = Uniform::new(0., 1.).unwrap();
        Box::new(move |v: Vec<f64>| v.iter().map(|x| d.pdf(*x)).product())
    }
}
