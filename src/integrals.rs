#![allow(dead_code)]
use rand::Rng;

#[allow(unused_imports)]
use super::utilities;

pub fn square_method<F>(f: F, x1: f64, x2: f64, step: f64) -> f64 
where F: Fn(f64) -> f64 {
    let mut x = x1;
    let mut res = 0.0;
    while x < x2 {
        x += step;
        res += f(x) * step;
    }
    res
}

pub fn trapezoid_method<F>(f: F, x1: f64, x2: f64, step: f64) -> f64
where F: Fn(f64) -> f64 {
    let mut x = x1;
    let mut res = 0.0;
    let mut previous = f(x);
    while x < x2 {
        x += step;
        let current = f(x);
        res += (previous + current) * step / 2.0;
        previous = current;
    }
    res
}

pub fn simpson_method<F>(f: F, x1: f64, x2: f64, n: usize) -> f64
where F: Fn(f64) -> f64 {
    let sectors = (x2 - x1) / n as f64;
    let mut res = 0.0;
    let mut x = x1;
    for _ in 0..n {
        res += (f(x) + 4.0 * f(x + sectors / 2.0) + f(x + sectors)) * sectors / 6.0;
        x += sectors;
    }
    res
}

pub fn montecarlo_method<F>(f: F, x1: f64, x2: f64, n: usize) -> f64
where F: Fn(f64) -> f64 {
    let mut rng = rand::thread_rng();
    let mut res = 0.0;
    for _ in 0..n {
        res += f(rng.gen_range(x1..x2));
    }
    res *= (x2 - x1).abs() / n as f64;

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    static F: fn(f64) -> f64 = |x| x.powi(3) + 2.0;

    #[test]
    fn test_integral_square_method1() {
        let result = square_method(F, 1.0, 4.0, 1.0);
        assert_eq!(result, 105.0);
    }
    
    #[test]
    fn test_integral_trapezoid_method1() {
        let result = trapezoid_method(F, 1.0, 4.0, 1.0);
        assert_eq!(result, 73.5);
    }
    
    #[test]
    fn test_integral_simpson_method1() {
        let result = simpson_method(F, 1.0, 5.0, 2);
        assert_eq!(result, 164.0);
    }
    
    #[test]
    fn test_integral_montecarlo_method1() {
        let result = montecarlo_method(F, 1.0, 5.0, 40000);
        println!("result = {}", result);
    }
}