#![allow(clippy::similar_names, dead_code, unused_imports)]

const EPSILON: f64 = 1e-10;

pub fn golden_ratio<F>(f: F, a: f64, b: f64) -> f64 
where
    F: Fn(f64) -> f64,
{
    let mut a = a;
    let mut b = b;
    let k = (5.0_f64.sqrt() - 1.0) / 2.0;
    let mut xl = b - k * (b - a);
    let mut xr = a + k * (b - a);

    while b - a > EPSILON {
        if f(xl) > f(xr) {
            a = xl;
        } else {
            b = xr;
        }
        xl = b - k * (b - a);
        xr = a + k * (b - a);
    }

    (a + b) / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_golden_ratio1() {
        let f = |x: f64| x.powi(2);
        let res = golden_ratio(f, -5.0, 5.0);
        let expected = 0.0;
        println!("{} {}", res, expected);
        assert!((res - expected).abs() < EPSILON);
    }
}