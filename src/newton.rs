#![allow(dead_code)]
#[allow(unused_imports)]
use super::utilities;
use std::fmt;

struct NewtonInterpolation {
    xs: Vec<f64>,
    layers: Vec<Vec<f64>>
}

impl NewtonInterpolation {
    fn new(xs: &[f64], ys: &[f64]) -> Self {
        let mut res = Self {
            xs: xs.to_vec(),
            layers: vec![ys.to_vec()]
        };
        res.calculate_layers();
        res
    }

    pub fn calculate(&mut self, x: f64) -> f64 {
        if self.layers.len() == 1 {
            self.calculate_layers();
        }
        let mut res = self.layers[0][0];
        for i in 1..self.layers.len() {
            let mut prod = 1.0;
            for j in 0..i {
                prod *= x-self.xs[j];
            }
            res += self.layers[i][0] * prod;
        }
        res
    }

    fn calculate_layers(&mut self) {
        let n = self.layers.len();
        for i in 0..self.layers[n-1].len()-1 {
            self.layers.push(vec![]);
            for j in 0..self.layers[n-1+i].len()-1 {
                let y1 = self.layers[n-1+i][j];
                let y2 = self.layers[n-1+i][j+1];
                let x1 = self.xs[j];
                let x2 = self.xs[j+i+1];
                self.layers[n+i].push((y2 - y1) / (x2 - x1));
            }
        }
    }

    pub fn add(&mut self, x: f64, y: f64) {
        self.xs.push(x);
        self.layers[0].push(y);
        self.layers.push(vec![]);
        for i in 1..self.layers.len() {
            let y1 = self.layers[i-1][self.layers[i-1].len()-2];
            let y2 = self.layers[i-1][self.layers[i-1].len()-1];
            self.layers[i].push((
                y2 - y1
            )/(
                self.xs[self.xs.len()-1] - self.xs[self.xs.len()-i-1]
            ));
        }
    }

}

impl fmt::Debug for NewtonInterpolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\nNewtonInterpolation {{")?;
        writeln!(f, "    xs: {:?}, ", self.xs)?;
        writeln!(f, "    layers: [")?;
        for i in 0..self.layers.len() {
            writeln!(f, "        {:?}, ", self.layers[i])?;
        }
        writeln!(f, "    ]")?;
        writeln!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utilities::parse_file;

    #[test]
    fn test_newton1() {
        let (xs, ys, x) = parse_file("data/test_newton.txt");
        let mut newton = NewtonInterpolation::new(&xs, &ys);
        println!("\nUnknown x, no add{:#?}", newton);
        let result = newton.calculate(x);
        let expected = 44.0;
        println!("result: {:.5}, expected: {}\n", result, expected);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_newton2() {
        let (mut xs, mut ys, x) = parse_file("data/test_newton.txt");
        let nx = xs.pop().unwrap();
        let ny = ys.pop().unwrap();
        let mut newton = NewtonInterpolation::new(&xs, &ys);
        newton.add(nx, ny);
        println!("\nUnknown x, add{:#?}", newton);
        let result = newton.calculate(x);
        let expected = 44.0;
        println!("result: {:.5}, expected: {}\n", result, expected);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_newton3() {
        let (mut xs, mut ys, x) = parse_file("data/test_newton.txt");
        let nx = xs.remove(0);
        let ny = ys.remove(0);
        let mut newton = NewtonInterpolation::new(&xs, &ys);
        newton.add(nx, ny);
        println!("\nUnknown x, add{:#?}", newton);
        let result = newton.calculate(x);
        let expected = 44.0;
        println!("result: {:.5}, expected: {}\n", result, expected);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_newton4() {
        let (xs, ys, _) = parse_file("data/test_newton.txt");
        let mut newton = NewtonInterpolation::new(&xs, &ys);
        println!("{:#?}", newton);
        let x = xs[2];
        let result = newton.calculate(x);
        let expected = ys[2];
        println!("result: {:.5}, expected: {}\n", result, expected);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_newton5() {
        let (xs, ys, _) = parse_file("data/test_newton.txt");
        let mut newton = NewtonInterpolation::new(&xs, &ys);
        println!("\nKnown x, no add{:#?}", newton);
        let x = xs[3];
        let result = newton.calculate(x);
        let expected = ys[3];
        println!("result: {:.5}, expected: {}\n", result, expected);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_newton6() {
        let (mut xs, mut ys, _) = parse_file("data/test_newton.txt");
        let nx = xs.remove(0);
        let ny = ys.remove(0);
        let mut newton = NewtonInterpolation::new(&xs, &ys);
        newton.add(nx, ny);
        println!("\nKnown x, add{:#?}", newton);
        let x = xs[2];
        let result = newton.calculate(x);
        let expected = ys[2];
        println!("result: {:.5}, expected: {}\n", result, expected);
        assert_eq!(result, expected);
    }
}