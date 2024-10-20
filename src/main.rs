use std::f64::consts::PI;

use statrs::distribution::Uniform;

use mclib::distributions::NDUniform;
use mclib::estimators::{BaseEstimator, MCEstimator};

fn test_function(x: f64) -> f64 {
    x.sin()
}

fn test_function2(x: Vec<f64>) -> f64 {
    f64::from(x[0] * x[0] + x[1] * x[1] <= 1.)
}

fn test_function3(x: Vec<f64>) -> f64 {
    f64::from(x[0] * x[0] + x[1] * x[1] + x[2] * x[2] <= 1.)
}

fn main() {
    let n = 10000;
    let start = 0.;
    let end = PI;
    let mut a = MCEstimator::new(n, Uniform::new(start, end).unwrap());
    let b = a.integrate(test_function);
    println!("{}", b);

    let dist = NDUniform::new(2).unwrap();
    let mut c = MCEstimator::new(n, dist);
    let d = c.integrate(test_function2);
    println!("{}", d);

    let dist2 = NDUniform::new(3).unwrap();
    let mut e = MCEstimator::new(n, dist2);
    let f = e.integrate(test_function3);
    println!("{}", f);
}
