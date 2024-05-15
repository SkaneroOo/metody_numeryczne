#![allow(clippy::similar_names, dead_code)]

pub fn choleski(coefficients: Vec<Vec<f64>>, constants: Vec<f64>) -> Option<Vec<f64>> {
    let n = coefficients.len();
    for i in 0..n {
        if coefficients[i].len() != n {
            return None;
        }
    }
    if constants.len() != n {
        return None;
    }

    for i in 0..n {
        for j in 0..i {
            if coefficients[i][j] != coefficients[j][i] {
                return None;
            }
        }
    }

    let mut l = vec![vec![0.0; n]; n];
    let mut lt = vec![vec![0.0; n]; n];

    for i in 0..n {
        let mut sum = 0.0;
        for j in 0..i {
            sum += l[i][j] * l[i][j];
        }
        let val = (coefficients[i][i] - sum).sqrt();
        l[i][i] = val;
        lt[i][i] = val;

        for j in i + 1..n {
            sum = 0.0;
            for k in 0..i {
                sum += l[i][k] * l[j][k];
            }
            l[j][i] = (coefficients[i][j] - sum) / l[i][i];
            lt[i][j] = l[j][i];
        }
    }

    println!("{:?}", l);

    println!("{:?}", lt);

    let mut ys = vec![0.0; n];

    for i in 0..n {
        let mut sum = 0.0;
        for j in 0..i {
            sum += l[i][j] * ys[j];
        }
        ys[i] = (constants[i] - sum) / l[i][i];
    }

    println!("{:?}", ys);

    let mut xs = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = 0.0;
        for j in i + 1..n {
            sum += lt[i][j] * xs[j];
        }
        xs[i] = (ys[i] - sum) / lt[i][i];
    }

    println!("{:?}", xs);


    Some(xs)
}

#[cfg(test)]
mod tests {
    use crate::sole::gauss::gauss1;

    use super::*;

    #[test]
    fn test_sole_choleski1() {
        let coefficients = vec![
            vec![4.0, -2.0, 2.0],
            vec![-2.0, 2.0, 2.0],
            vec![2.0,  2.0, 14.0]
        ];
        let constants = vec![-6.0, 4.0, 0.0]; // potencjalny błąd w skrypcie
        let res = choleski(coefficients, constants);
        assert!(res.is_some());
        let res = res.unwrap_or_default();
        println!("{res:?}");
    }

    #[test]
    fn test_sole_choleski2() {
        let coefficients = vec![
            vec![3.0, -4.0, 4.0, -4.0],
            vec![1.5, -1.0, 2.0, -2.0],
            vec![1.5, -0.5, 0.0, -3.0],
            vec![4.5, -5.5, 4.0, -9.0]
        ];
        let constants = vec![-9.0, -3.5, -2.0, -14.0]; // macież nie jest symetryczna, więc nie jest możliwy rozkład Choleskiego
        let res = choleski(coefficients, constants);
        assert!(res.is_none());
        let res = res.unwrap_or_default();
        println!("{res:?}");
    }

    #[test]
    fn test_sole_gauss_extra() {
        let coefficients = vec![
            vec![3.0, -4.0, 4.0, -4.0],
            vec![1.5, -1.0, 2.0, -2.0],
            vec![1.5, -0.5, 0.0, -3.0],
            vec![4.5, -5.5, 4.0, -9.0]
        ];
        let constants = vec![-9.0, -3.5, -2.0, -14.0];
        let res = gauss1(coefficients, constants);
        assert!(res.is_some());
        let res = res.unwrap_or_default();
        println!("{res:?}");
    }

    #[test]
    fn test_sole_choleski3() {
        let coefficients = vec![
            vec![1.0, 2.0, 3.0],
            vec![2.0, 8.0, 10.0],
            vec![3.0,  10.0, 22.0]
        ];
        let constants = vec![1.0, 3.0, 7.0]; 
        let res = choleski(coefficients, constants);
        assert!(res.is_some());
        let res = res.unwrap_or_default();
        println!("{res:?}");
    }
}