#![allow(clippy::similar_names, dead_code, unused_imports)]

pub fn euler<F>(f: F, x0: f64, y0: f64, b: f64, h: f64) -> Vec<f64> 
where
    F: Fn(f64, f64) -> f64,
{
    let mut ys = vec![y0];
    let mut x = x0;

    for _ in 0..((b - x0) / h).ceil() as usize {
        x += h;
        ys.push(ys.last().unwrap() + h * f(x, ys[ys.len() - 1]));
    }

    // ys.remove(0);

    ys
}

pub fn heun<F>(f: F, x0: f64, y0: f64, b: f64, h: f64) -> Vec<f64> 
where
    F: Fn(f64, f64) -> f64,
{
    let mut ys = vec![y0];
    let mut x = x0;

    for _ in 0..((b - x0) / h).ceil() as usize {
        x += h;
        let y = ys[ys.len() - 1];
        ys.push(y + h / 2.0 * 
            (f(x, y) + f(x + h, y + h * f(x, y)))
        );
    }

    ys
}

pub fn rk4<F>(f: F, x0: f64, y0: f64, b: f64, h: f64) -> Vec<f64> 
where
    F: Fn(f64, f64) -> f64,
{
    let mut ys = vec![y0];
    let mut x = x0;

    for _ in 0..((b - x0) / h).ceil() as usize {
        x += h;
        let y = ys[ys.len() - 1];
        let mut ks = vec![0.0; 4];
        ks[0] = h * f(x, y);
        ks[1] = h * f(x + 0.5 * h, y + 0.5 * ks[0]);
        ks[2] = h * f(x + 0.5 * h, y + 0.5 * ks[1]);
        ks[3] = h * f(x + h, y + ks[2]);
        ys.push(y + (ks[0] + 2.0 * ks[1] + 2.0 * ks[2] + ks[3]) / 6.0);
    }

    ys
}
#[cfg(test)]
mod tests {
    use super::*;
    use plotly::common::Mode;
    use plotly::{Plot, Scatter};

    #[test]
    fn test_euler1() {
        let f = |_: f64, y: f64| y;
        let ys = euler(f, 0.0, 1.0, 0.4, 0.1);
        assert!(ys.len() > 0);
        println!("Test 1: {:?}", ys);
    }

    #[test]
    fn test_euler2() {
        let f = |x: f64, y: f64| y;
        let x0 = 0.0;
        let y0 = 1.0;
        let b = 4.0;
        let h = 0.1;
        let ys = euler(f, x0, y0, b, h);
        let mut x = Vec::new();
        x.push(0.0);
        for _ in 0..((b - x0) / h) as usize {
            x.push(x.last().unwrap() + h);
        }
        let mut y = Vec::new();
        for i in x.iter() {
            y.push(i.exp());
        }
        assert!(ys.len() > 0);
        let expected = Scatter::new(x.clone(), y).name("Expected").mode(Mode::Lines);
        let calculated = Scatter::new(x, ys.clone()).name("Calculated").mode(Mode::Lines);

        let mut plot = Plot::new();
        plot.add_trace(expected);
        plot.add_trace(calculated);
        plot.show();
        println!("Test 2: {:?}", ys);
        println!("Test 2: {}", ys.last().unwrap());
    }

    #[test]
    fn test_heun1() {
        let f = |x: f64, y: f64| x.cos() - x.sin() - y;
        let x0 = 0.0;
        let y0 = 2.0;
        let b = 12.0;
        let h = 0.1;
        let ys = heun(f, x0, y0, b, h);
        let mut x = Vec::new();
        x.push(0.0);
        for _ in 0..((b - x0) / h) as usize {
            x.push(x.last().unwrap() + h);
        }
        let mut y = Vec::new();
        for i in x.iter() {
            y.push((-i).exp() + i.cos());
        }
        assert!(ys.len() > 0);
        let expected = Scatter::new(x.clone(), y).name("Expected").mode(Mode::Lines);
        let calculated = Scatter::new(x, ys.clone()).name("Calculated").mode(Mode::Lines);

        let mut plot = Plot::new();
        plot.add_trace(expected);
        plot.add_trace(calculated);
        plot.show();
        println!("Test 1: {:?}", ys);
    }

    #[test]
    fn test_rk1() {
        let f = |x: f64, y: f64| x + y;
        // let ys = rk4(f, 0.0, 1.0, 0.2, 0.1);
        // assert!(ys.len() > 0);
        let x0 = 0.0;
        let y0 = 1.0;
        let b = 10.0;
        let h = 0.01;
        let ys = rk4(f, x0, y0, b, h);
        let mut x = Vec::new();
        x.push(0.0);
        for _ in 0..((b - x0) / h) as usize {
            x.push(x.last().unwrap() + h);
        }
        let mut y = Vec::new();
        for i in x.iter() {
            y.push(i.exp() - i);
        }
        assert!(ys.len() > 0);
        let expected = Scatter::new(x.clone(), y).name("Expected").mode(Mode::Lines);
        let calculated = Scatter::new(x, ys.clone()).name("Calculated").mode(Mode::Lines);

        let mut plot = Plot::new();
        plot.add_trace(expected);
        plot.add_trace(calculated);
        plot.show();
        println!("Test 1: {:?}", ys);
    }

}