#![allow(clippy::similar_names, dead_code)]

pub fn lu(coefficients: Vec<Vec<f64>>, constants: Vec<f64>) -> Option<Vec<f64>> {
    let n = coefficients.len();
    for i in 0..n {
        if coefficients[i].len() != n {
            return None;
        }
    }
    if constants.len() != n {
        return None;
    }

    let mut l = vec![vec![0.0; n]; n];
    let mut u = vec![vec![0.0; n]; n];

    for i in 0..n {
        l[i][i] = 1.0;
    }

    let mut sum;
    for i in 0..n {
        for j in i..n {
            sum = 0.0;
            for k in 0..i {
                sum += l[i][k] * u[k][j];
            }
            u[i][j] = coefficients[i][j] - sum;

            if i == j {
                continue;
            }

            sum = 0.0;
            for k in 0..i {
                sum += l[j][k] * u[k][i];
            }
            l[j][i] = (coefficients[j][i] - sum) / u[i][i];
        }
    }

    // println!("{:?}", u);
    // println!("{:?}", l);


    let mut ys = vec![0.0; n];

    for i in 0..n {
        let mut sum = 0.0;
        for j in 0..i {
            sum += l[i][j] * ys[j];
        }
        ys[i] = (constants[i] - sum) / l[i][i];
    }

    // println!("{:?}", ys);

    let mut xs = vec![0.0; n];

    for i in (0..n).rev() {
        let mut sum = 0.0;
        for j in i + 1..n {
            sum += u[i][j] * xs[j];
        }
        xs[i] = (ys[i] - sum) / u[i][i];
    }

    // println!("{:?}", xs);


    Some(xs)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sole_lu0() {
        let coefficients = vec![
            vec![2.0, -3.0, -1.0],
            vec![4.0, -7.0, 2.0],
            vec![-2.0,  -1.0, 15.0]
        ];
        let constants = vec![10.0, 9.0, -48.0];
        let res = lu(coefficients, constants);
        assert!(res.is_some());
        let res = res.unwrap_or_default();
        println!("{res:?}");
    }

    #[test]
    fn test_sole_lu1() {
        let coefficients = vec![
            vec![4.0, -2.0, 2.0],
            vec![-2.0, 2.0, 2.0],
            vec![2.0,  2.0, 14.0]
        ];
        let constants = vec![-6.0, 4.0, 0.0];
        let res = lu(coefficients, constants);
        assert!(res.is_some());
        let res = res.unwrap_or_default();
        println!("{res:?}");
    }

    #[test]
    fn test_sole_lu2() {
        let coefficients = vec![
            vec![3.0, -4.0, 4.0, -4.0],
            vec![1.5, -1.0, 2.0, -2.0],
            vec![1.5, -0.5, 0.0, -3.0],
            vec![4.5, -5.5, 4.0, -9.0]
        ];
        let constants = vec![-9.0, -3.5, -2.0, -14.0];
        let res = lu(coefficients, constants);
        assert!(res.is_some());
        let res = res.unwrap_or_default();
        println!("{res:?}");
    }

    #[test]
    fn test_sole_lu3() {
        let coefficients = vec![
            vec![1.0, 2.0, 3.0],
            vec![2.0, 8.0, 10.0],
            vec![3.0,  10.0, 22.0]
        ];
        let constants = vec![1.0, 3.0, 7.0]; 
        let res = lu(coefficients, constants);
        assert!(res.is_some());
        let res = res.unwrap_or_default();
        println!("{res:?}");
    }
}