mod mcestimator;
pub use mcestimator::MCEstimator;

/// Base trait for estimator/integrator
pub trait BaseEstimator<X> {
    /// Integrate given a function
    fn integrate(&mut self, integrand: fn(X) -> f64) -> f64;
}
