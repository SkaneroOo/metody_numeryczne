#![allow(dead_code)]
#[allow(unused_imports)]
use super::utilities;

#[derive(Debug)]
struct NewtonInterpolation {
    xs: Vec<f64>,
    layers: Vec<Vec<f64>>
}

impl NewtonInterpolation {
    fn new(xs: &[f64], ys: &[f64]) -> NewtonInterpolation {
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
                self.layers[n+i].push((
                    y2-y1
                )/(
                    self.xs[j+i+1]-self.xs[j]
                ));
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

#[cfg(test)]
mod tests {
    use super::*;
    use utilities::parse_file;

    #[test]
    fn test_newton() {
        let (xs, ys, x) = parse_file("data/test_newton.txt");
        let mut newton = NewtonInterpolation::new(&xs, &ys);
        println!("{:#?}", newton);
        let result = newton.calculate(x);
        assert_eq!(result, 44.0);
    }

    #[test]
    fn test_newton2() {
        let (mut xs, mut ys, x) = parse_file("data/test_newton.txt");
        let nx = xs.pop().unwrap();
        let ny = ys.pop().unwrap();
        let mut newton = NewtonInterpolation::new(&xs, &ys);
        newton.add(nx, ny);
        println!("{:#?}", newton);
        let result = newton.calculate(x);
        assert_eq!(result, 44.0);
    }
}