use std::fs;

#[allow(dead_code)]
pub fn parse_file(filename: &str) -> (Vec<f64>, Vec<f64>, Option<f64>) {
    let mut data = match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(e) => panic!("{}", e)
    };
    data = data.replace("\r\n", "\n");
    let mut lines = data.lines();
    let n = match lines.next().unwrap().parse::<f64>() {
        Ok(n) => Some(n),
        Err(_) => None
    };
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for line in lines {
        let mut tokens = line.split_whitespace();
        let x = tokens.next().unwrap().parse::<f64>().unwrap();
        let y = tokens.next().unwrap().parse::<f64>().unwrap();
        xs.push(x);
        ys.push(y);
    }

    (xs, ys, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let (xs, ys, n) = parse_file("data/read_test.txt");
        assert_eq!(xs, vec![1.0, 2.0, 3.0]);
        assert_eq!(ys, vec![1.0, 4.0, 9.0]);
        assert_eq!(n, Some(3.0));
    }
}