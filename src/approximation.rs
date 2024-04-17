#![allow(clippy::similar_names, clippy::suboptimal_flops, dead_code)]

type A0 = f64;
type A1 = f64;
type R = f64;

pub fn approximation(xs: &[f64], ys: &[f64]) -> (A0, A1, R) {
    assert!(xs.len() == ys.len());
    let n = xs.len();

    let mut sxy = 0.0;
    let mut sx = 0.0;
    let mut sy = 0.0;
    let mut sx2 = 0.0;
    let mut sy2 = 0.0;

    for i in 0..n {
        sxy += xs[i] * ys[i];
        sx += xs[i];
        sy += ys[i];
        sx2 += xs[i] * xs[i];
        sy2 += ys[i] * ys[i];
    }

    #[allow(clippy::cast_precision_loss)]
    let n = n as f64;
    let s2x = sx * sx;
    let s2y = sy * sy;
    let a0 = (sy * sx2 - sx * sxy) / (n * sx2 - s2x);
    let a1 = (n * sxy - sx * sy) / (n * sx2 - s2x);
    let r = (n * sxy - sx * sy) / (
        (n * sx2 - s2x).sqrt() * (n * sy2 - s2y).sqrt()
    );

    (a0, a1, r)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::parse_file;

    #[test]
    fn test_approximation1() {
        let (xs, ys, _) = parse_file("data/test_approximation.txt");
        let (a0, a1, r) = approximation(&xs, &ys);
        println!("\na0: {a0}\na1: {a1}\nr: {r}\n");
        assert!((-15.0 - a0).abs() <= 0.001);
        assert!((7.5 - a1).abs() <= 0.001);
    }

    #[test]
    fn test_approximation2() {
        let (xs, ys, _) = parse_file("data/test_approximation2.txt");
        let (a0, a1, r) = approximation(&xs, &ys);
        println!("\na0: {a0}\na1: {a1}\nr: {r}\n");
        assert!((28.17 - a0).abs() <= 0.005);
        assert!((12.65 - a1).abs() <= 0.005);
    }
}