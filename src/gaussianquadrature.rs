#![allow(dead_code)]
#[allow(unused_imports)]
use super::utilities;

const WEIGHTS: [f64; 2] = [1.0, 1.0];

const POINTS: [f64; 2] = [0.5773502692, -0.5773502692];

pub fn gaussian_quadrature_2d(xs: &[f64], ys: &[f64]) -> f64 {
    assert!(xs.len() == ys.len() && xs.len() == 4);

    let mut res = 0.0;

    let mut p_ksi = [[0.0; 4]; 2];
    let mut p_ni = [[0.0; 4]; 2];

    for i in 0..2 {
        p_ksi[i][0] = -0.25 * (1.0 - POINTS[i]);
        p_ksi[i][1] =  0.25 * (1.0 - POINTS[i]);
        p_ksi[i][2] =  0.25 * (1.0 + POINTS[i]);
        p_ksi[i][3] = -0.25 * (1.0 + POINTS[i]);

        p_ni[i][0] = -0.25 * (1.0 - POINTS[i]);
        p_ni[i][1] = -0.25 * (1.0 + POINTS[i]);
        p_ni[i][2] =  0.25 * (1.0 + POINTS[i]);
        p_ni[i][3] =  0.25 * (1.0 - POINTS[i]);
    }

    for i in 0..2 {
        for j in 0..2 {
            let mut dxksi = 0.0;
            let mut dyksi = 0.0;
            let mut dxni = 0.0;
            let mut dyni = 0.0;
            for k in 0..4 {
                dxksi += p_ksi[j][k] * xs[k];
                dyksi += p_ksi[j][k] * ys[k];
                dxni += p_ni[i][k] * xs[k];
                dyni += p_ni[i][k] * ys[k];
            }

            res += (dxksi * dyni - dxni * dyksi).abs() * WEIGHTS[i] * WEIGHTS[j];
        }
    }

    res
}

#[cfg(test)]
mod tests {

    use super::*;
    use utilities::parse_file;

    #[test]
    fn test_gaussian_quadrature1() {
        let (xs, ys, expected) = parse_file("data/test_gaussian_quadrature.txt");
        let res = gaussian_quadrature_2d(&xs, &ys);
        println!("Result: {}\t\tExpected: {}\n", res, expected);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_gaussian_quadrature2() {
        let (xs, ys, expected) = parse_file("data/test_gaussian_quadrature2.txt");
        let res = gaussian_quadrature_2d(&xs, &ys);
        println!("Result: {}\t\tExpected: {}\n", res, expected);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_gaussian_quadrature3() {
        let (xs, ys, expected) = parse_file("data/test_gaussian_quadrature3.txt");
        let res = gaussian_quadrature_2d(&xs, &ys);
        println!("Result: {}\t\tExpected: {}\n", res, expected);
        assert_eq!(res, expected);
    }

}

// https://www.mathopenref.com/coordpolygonareacalc.html