#![allow(clippy::similar_names, dead_code)]

pub static EPS: f64 = 1e-6;

pub fn bisection<F>(f: F, x1: f64, x2: f64) -> Option<f64>
where F: Fn(f64) -> f64 {
    if f(x1) * f(x2) > 0.0 {
        return None
    }

    if f(x1) == 0.0 {
        return Some(x1)
    }

    if f(x2) == 0.0 {
        return Some(x2)
    }

    let mut a = x1;
    let mut b = x2;
    let mut x0 = (a + b) / 2.0;

    if f(x0) == 0.0 {
        return Some(x0)
    }

    while (a - b).abs() >= EPS {
        if f(a) * f(x0) < 0.0 {
            b = x0;
        } else if f(b) * f(x0) < 0.0 {
            a = x0;
        }
        x0 = (a + b) / 2.0;
        if f(x0) == 0.0 {
            return Some(x0)
        }
    }

    Some(x0)
}

pub fn newton_raphson<F>(f: F, df: F, x1: f64, x2: f64) -> Option<f64>
where F: Fn(f64) -> f64 {
    if f(x1) * f(x2) > 0.0 {
        return None
    }

    if f(x1) == 0.0 {
        return Some(x1)
    }

    if f(x2) == 0.0 {
        return Some(x2)
    }

    nr(f, df, (x1 + x2) / 2.0)
}

fn nr<F>(f: F, df: F, x: f64) -> Option<f64>
where F: Fn(f64) -> f64 {
    let fx = f(x);
    if fx.abs() < EPS {
        return Some(x)
    }
    let dfx = df(x);
    if dfx.abs() < EPS {
        return None;
    }
    let xn = x - fx / dfx;
    if xn.abs() < EPS {
        return Some(x);
    }
    nr(f, df, xn)
}

#[cfg(test)]
mod tests {

    use super::*;
    // use crate::utilities::parse_file;

    const F: fn(f64) -> f64 = |x| x.powi(2) - 2.0;
    const FP: fn(f64) -> f64 = |x| 2.0 * x;

    #[test]
    fn test_nonlinear_bisection1() {
        let res = bisection(F, 0.0, 4.0);
        println!("Bisection1: {res:?}");
        assert!(res.is_some());
    }

    #[test]
    fn test_nonlinear_bisection2() {
        let res = bisection(F, 0.0, -4.0);
        println!("Bisection2: {res:?}");
        assert!(res.is_some());
    }

    #[test]
    fn test_nonlinear_newton_raphson1() {
        let res = newton_raphson(F, FP, 0.0, 4.0);
        println!("Newton-Raphson1: {res:?}");
        assert!(res.is_some());
    }

    #[test]
    fn test_nonlinear_newton_raphson2() {
        let res = newton_raphson(F, FP, 0.0, -4.0);
        println!("Newton-Raphson2: {res:?}");
        assert!(res.is_some());
    }

    const F2: fn(f64) -> f64 = |x| x.powi(3) + 3.0 * x + 1.0;
    const FP2: fn(f64) -> f64 = |x| 3.0 * x.powi(2) + 3.0;

    #[test]
    fn test_nonlinear_newton_raphson3() {
        let res = newton_raphson(F2, FP2, -5.0, 5.0);
        println!("Newton-Raphson3: {res:?}");
        assert!(res.is_some());
    }
}