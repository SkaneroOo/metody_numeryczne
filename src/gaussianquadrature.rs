#![allow(dead_code)]
#[allow(unused_imports)]
use super::utilities;

const WEIGHTS: [f64; 2] = [1.0, 1.0];

const POINTS: [f64; 2] = [0.5773502692, -0.5773502692];

pub fn gaussian_quadrature(xs: &[f64], ys: &[f64]) -> f64 {
    assert!(xs.len() == ys.len() && xs.len() == 4);

    let mut res = 0.0;

    res
}

#[cfg(test)]
mod tests {

}