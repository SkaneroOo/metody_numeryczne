#![allow(clippy::similar_names, dead_code, unused_imports)]

#[derive(Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows]
        }
    }

    pub fn det(&self) -> Option<f64> {
        if self.rows != self.cols {
            return None
        }

        match self.rows {
            0 => Some(0.0),
            1 => Some(self.data[0][0]),
            2 => Some(self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0]),
            _ => {
                let mut det = 0.0;
                let mut sign = 1.0;

                for i in 0..self.cols {
                    det += self.data[0][i] * self.submatrix(0, i).det().expect("unexpected error") * sign;
                    sign *= -1.0;
                }

                Some(det)
            }
        }
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut m = Matrix::new(self.rows - 1, self.cols - 1);

        let mut ip = 0;
        let mut jp = 0;
        for i in 0..self.rows {
            if i > row {
                ip = 1;
            } else {
                ip = 0;
            }
            for j in 0..self.cols {
                if j > col {
                    jp = 1;
                } else {
                    jp = 0;
                }
                if i != row && j != col {
                    m.data[i - ip][j - jp] = self.data[i][j];
                }
            }
        }
        m
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix1() {
        let m = Matrix {
            rows: 4,
            cols: 4,
            data: vec![
                vec![2.0, 4.0, 2.0, 5.0],
                vec![1.0, 5.0, 2.0, 6.0],
                vec![8.0, 5.0, 3.0, 2.0],
                vec![0.0, 1.0, 3.0, 6.0]
            ]
        };
        println!("{}", m.det().unwrap());
    }

    #[test]
    fn test_submtx() {
        let m = Matrix {
            rows: 4,
            cols: 4,
            data: vec![
                vec![2.0, 4.0, 2.0, 5.0],
                vec![1.0, 5.0, 2.0, 6.0],
                vec![8.0, 5.0, 3.0, 2.0],
                vec![0.0, 1.0, 3.0, 6.0]
            ]
        };
        println!("{:?}", m.submatrix(0,0));
    }
}