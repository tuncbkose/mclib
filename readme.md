# MCLib

A library implementing Monte Carlo (and other numerical?) integration methods.
I just started learning Rust, so if things look weird, please let me know.

## Usage example
```rust
use std::f64::consts::PI;
use statrs::distribution::Normal;
use mclib::MCEstimator;

fn test_function(x: f64) -> f64 {
    x.sin()
}

fn main() {
    let n_samples = 10000;
    let start = 0.;
    let end = PI;
    let mut a = MCEstimator::new(n_samples, Uniform::new(start, end).unwrap());
    let estimate = a.integrate(test_function);
    println!("{}", estimate);
}
```

## Todos
- Support multivariate integrands
  - Add bounds support to multivariate uniform
- Add sampling methods
- Add non-MC estimators
- Tests
- Proper documentation?
