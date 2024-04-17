#![allow(dead_code)]

pub fn interpolation(x: f64, xs: &[f64], ys: &[f64]) -> Option<f64> {
    if xs.len() != ys.len() {
        return None;
    }

    let mut sum = 0.0;

    for i in 0..xs.len() {
        let mut prod = 1.0;
        for j in 0..xs.len() {
            if i != j {
                prod *= (x - xs[j]) / (xs[i] - xs[j]);
            }
        }
        sum += prod * ys[i];
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::parse_file;

    #[test]
    fn test_lagrange() {
        let (xs, ys, x) = parse_file("data/test_lagrange.txt");
        let x = x.expect("No x provided");
        let result = interpolation(x, &xs, &ys);
        assert_eq!(result, Some(3.0));
    }
}