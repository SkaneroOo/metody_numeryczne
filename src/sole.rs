#![allow(clippy::similar_names, dead_code)]

// Systems of Linear Equations

pub fn gauss(coefficients: Vec<Vec<f64>>, constants: Vec<f64>) -> Option<Vec<f64>> {
    let n = coefficients.len();
    for i in 0..n {
        if coefficients[i].len() != n {
            return None;
        }
    }
    if constants.len() != n {
        return None;
    }

    let mut coefficients = coefficients;
    let mut constants = constants;

    let mut xs = vec![0.0; n];

    for i in 0..n-1 {
        for j in i+1..n {
            let ratio = coefficients[j][i] / coefficients[i][i];
            for k in i..n {
                coefficients[j][k] -= ratio * coefficients[i][k];
            }
            constants[j] -= ratio * constants[i];
        }
    }

    for i in (0..n).rev() {
        let mut sum = 0.0;
        for j in i+1..n {
            sum += coefficients[i][j] * xs[j];
        }
        xs[i] = (constants[i] - sum) / coefficients[i][i];
    }

    Some(xs)
} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sole_gauss1() {
        let coefficients = vec![
            vec![4.0, -2.0, 4.0, -2.0],
            vec![3.0,  1.0, 4.0,  2.0],
            vec![2.0,  4.0, 2.0,  1.0],
            vec![2.0, -2.0, 4.0,  2.0]
        ];
        let constants = vec![8.0,   7.0,  10.0,   2.0];
        let res = gauss(coefficients, constants);
        assert!(res.is_some());
        let res = res.unwrap_or_default();
        println!("{res:?}");
    }

    #[test]
    fn test_sole_gauss2() {
        let coefficients = vec![
            vec![10.0, -7.0, 0.0],
            vec![-3.0,  2.0, 6.0],
            vec![ 5.0, -1.0, 5.0]
        ];
        let constants = vec![6.0,   4.0,   3.0];
        let res = gauss(coefficients, constants);
        assert!(res.is_some());
        let res = res.unwrap_or_default();
        println!("{res:?}");
    }

    #[test]
    fn test_sole_gauss3() {
        let coefficients = vec![
            vec![ 5.0,   1.0,   7.0,  -7.0,   6.0,   9.0,  -8.0,   8.0,   6.0,  -1.0,  -8.0,  -7.0,   2.0,  -4.0,  -8.0,   0.0,  -6.0,  -7.0, -10.0,  -8.0],
            vec![ 7.0,  -6.0,  -6.0,  -3.0,  10.0,   8.0,  -8.0,  -4.0,   2.0,  -4.0,   0.0,   7.0,   1.0,   7.0,   2.0,   2.0,   4.0,  -4.0,   3.0,   7.0],
            vec![ 9.0,  -4.0,  -7.0,   4.0,  -9.0,   8.0,   4.0,   9.0,   5.0,   5.0,  -4.0,  -7.0,   0.0,  -6.0,   4.0,   2.0,   1.0,   3.0,  -2.0,   3.0],
            vec![ 5.0,   3.0,  -4.0,   1.0, -10.0,  -9.0,   6.0,   9.0,  -9.0,  -5.0,   0.0, -10.0,  -8.0,   9.0,   0.0,   5.0,   0.0,   7.0,  -4.0,  10.0],
            vec![-3.0,  -9.0,  -9.0,  10.0,   7.0,   1.0,   2.0,   3.0,   5.0,   8.0,  -2.0,  -1.0,   9.0,  -1.0,   1.0,   1.0,  -2.0,  -8.0,   7.0,   4.0],
            vec![ 3.0,  -3.0,   5.0,  -8.0,  -1.0,  -4.0,   8.0,  -1.0,   7.0,  10.0, -10.0,  -1.0,   4.0,   0.0,   6.0,  -4.0,  -9.0,  -6.0,  -4.0,   0.0],
            vec![ 3.0,   0.0,  10.0,   3.0,  -6.0,   7.0,  -7.0,  -8.0,   9.0,  -4.0,  -7.0,   6.0,   9.0, -10.0,   2.0,  -5.0,   3.0,  -1.0,  -3.0, -10.0],
            vec![ 7.0,   2.0,  -4.0,   4.0,   0.0,   3.0,  -6.0, -10.0,   0.0,  -4.0,   4.0,   1.0,  -9.0,  -2.0,  -6.0,   9.0,   1.0,   5.0, -10.0,   8.0],
            vec![ 9.0,  -7.0,   6.0,   6.0,   5.0,   0.0,  -7.0,  -7.0,   8.0,   6.0,  -7.0,  -6.0,  -1.0,  -9.0,  -1.0,  -9.0,   1.0,  -1.0,   1.0,  -9.0],
            vec![ 6.0,  -7.0,   5.0,   8.0,  -3.0, -10.0,   1.0,  -4.0,  -1.0,  -7.0,   6.0,   4.0,   5.0,  -5.0,  10.0,   1.0, -10.0,   7.0,  -6.0,  -7.0],
            vec![ 4.0,  -8.0,  -4.0,  -6.0,   2.0,  -5.0,   2.0,   6.0,   2.0,   1.0,  -9.0,  -4.0,   9.0,  -6.0,  -2.0,  -8.0,  -3.0,   1.0,  -1.0,   3.0],
            vec![ 3.0,  -7.0,   2.0,  -5.0, -10.0,  -2.0,   2.0, -10.0,   4.0,   6.0, -10.0,  -5.0,  -9.0,  -1.0,   7.0,   5.0,  -1.0,  -2.0,  -4.0,  -4.0],
            vec![ 7.0,   3.0,   0.0,  -2.0,   6.0,  -3.0,  10.0,   4.0,  -7.0,   1.0,   4.0,   5.0,  -2.0,  -5.0, -10.0,   5.0,   7.0,   5.0,  -9.0, -10.0],
            vec![ 1.0, -10.0,  -4.0,   4.0,  10.0,   3.0,   4.0,  -2.0,  -5.0,  10.0,  -2.0,  -3.0,  -1.0,  10.0,   4.0,   5.0,   9.0,   8.0,  -6.0,  -5.0],
            vec![-3.0,  -1.0,   6.0,  -5.0,   1.0,  -6.0,   1.0,  -3.0,   9.0,   4.0,   6.0,   0.0,  -6.0, -10.0,   2.0,   5.0,  -2.0,  -2.0,   2.0,   9.0],
            vec![-9.0,   2.0,  -2.0,   1.0,  -9.0,   7.0,  10.0,   2.0,   3.0,  -8.0,  -8.0,   5.0,   9.0, -10.0,   8.0,  -8.0,   6.0,   1.0,  -4.0, -10.0],
            vec![ 4.0,  -7.0,   0.0,   5.0,  -7.0,  10.0,   3.0,  -4.0,   9.0,  -9.0,   3.0,  -2.0,  -1.0,  10.0,   7.0,  -1.0, -10.0,   5.0,   7.0,  -5.0],
            vec![-7.0,   9.0,   8.0,   1.0,   0.0,   0.0,   0.0,   5.0,  -1.0,  -1.0, -10.0, -10.0,   4.0,  -9.0,   3.0,   5.0,  -2.0,   2.0,  -5.0,  -9.0],
            vec![-5.0,   6.0,   3.0, -10.0,  -4.0,   0.0,   5.0,   2.0,   9.0,   3.0,   4.0,   3.0,   8.0,   1.0,   1.0,   5.0,   1.0,  -8.0, -10.0,  -8.0],
            vec![ 4.0,  -9.0,  -4.0,   9.0,  -1.0,  10.0,   9.0, -10.0,  -8.0,   2.0,   4.0,   7.0,   8.0,   1.0,  -5.0,  -7.0,   5.0,   3.0,   5.0,  -8.0],
        ];
        let constants = vec![-8.0, 2.0, 5.0, 7.0, 5.0, -9.0, 0.0, 9.0, 4.0, -5.0, 8.0, -9.0, 1.0, 3.0, -2.0, -2.0, 6.0, 10.0, 2.0, -5.0];

        /*
        expected:
        [
             0.42483177  1.05426056 -0.89539233 -0.06691239  0.41554584
            -0.52006665  0.10889383 -0.67418602  1.06754043 -0.37442787
            -0.09826593 -1.07958079  1.04631257  0.13620134 -0.25838214
             0.48234027  0.49453971  1.02800194  0.56439505 -0.01779573
        ]
        */

        let res = gauss(coefficients, constants);
        assert!(res.is_some());
        let res = res.unwrap_or_default();
        println!("[");
        for i in 0..4 {
            print!("    ");
            for j in 0..5 {
                if res[i * 5 + j] >= 0.0 {
                    print!(" ");
                }
                print!("{:.8} ", res[i * 5 + j]);
            }
            println!();
        }
        println!("]");

    }
}