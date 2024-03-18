#[allow(unused_imports)]
use super::utilities;

#[allow(dead_code)]
pub fn lagrange_interpolation(x: f64, xs: &[f64], ys: &[f64]) -> f64 {
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
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use utilities::parse_file;

    #[test]
    fn test_lagrange() {
        let (xs, ys, x) = parse_file("data/test_lagrange.txt");
        let result = lagrange_interpolation(x, &xs, &ys);
        assert_eq!(result, 3.0);
    }
}