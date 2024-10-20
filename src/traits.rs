use rand::distributions::Distribution;
use statrs::distribution::*;

use crate::impl_dist;

/// Distribution wrapper trait: Distribution<T> should give sampling and density is implemented by user.
pub trait Dist<T>: Distribution<T> {
    fn density(&self) -> Box<dyn Fn(T) -> f64 + '_>;
}

impl_dist!(
    Beta,
    Cauchy,
    Chi,
    ChiSquared,
    Erlang,
    Exp,
    FisherSnedecor,
    Gamma,
    InverseGamma,
    Laplace,
    LogNormal,
    Normal,
    Pareto,
    StudentsT,
    Triangular,
    Uniform,
    Weibull
);
