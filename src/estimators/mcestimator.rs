use rand::rngs::ThreadRng;

use crate::estimators::BaseEstimator;
use crate::traits::Dist;

pub struct MCEstimator<D> {
    n_samples: u64,
    rng: ThreadRng,
    dist: D,
}
/// Simple Monte Carlo integrator
impl<D> MCEstimator<D> {
    /// Create an instance of MCEstimator
    pub fn new(n_samples: u64, dist: D) -> Self {
        MCEstimator {
            n_samples,
            rng: rand::thread_rng(),
            dist,
        }
    }
}

impl<T, D> BaseEstimator<T> for MCEstimator<D>
where
    T: Clone,
    D: Dist<T>,
{
    fn integrate(&mut self, integrand: fn(T) -> f64) -> f64 {
        (0..self.n_samples)
            .map(|_| {
                let s = self.dist.sample(&mut self.rng);
                integrand(s.clone()) / self.dist.density()(s)
            })
            .sum::<f64>()
            / self.n_samples as f64
    }
}
