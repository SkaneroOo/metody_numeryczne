use std::fs;

#[allow(dead_code)]
pub fn parse_file(filename: &str) -> (Vec<f64>, Vec<f64>, Option<f64>) {
    let mut data = match fs::read_to_string(filename) {
        Ok(data) => data,
        Err(e) => panic!("{}", e)
    };
    data = data.replace("\r\n", "\n");
    let mut lines = data.lines();
    let n = lines.next().unwrap_or_default().parse::<f64>().ok();
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for line in lines {
        let mut tokens = line.split_whitespace();
        let x = tokens.next().expect("Invalid file format").parse::<f64>().expect("Invalid file format");
        let y = tokens.next().expect("Invalid file format").parse::<f64>().expect("Invalid file format");
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