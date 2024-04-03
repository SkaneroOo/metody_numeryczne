mod lagrange;
mod utilities;

fn lagrange_printable() {
    let (xs, ys, x) = utilities::parse_file("data/read_test.txt");

    let x = x.unwrap();
    println!("Punkty:");
    for i in 0..xs.len() {
        println!("({}, {})", xs[i], ys[i]);
    }
    let result = lagrange::lagrange_interpolation(x, &xs, &ys);
    println!("f({}) -> {}", x, result);
    println!();
    let (xs, ys, x) = utilities::parse_file("data/test_lagrange.txt");

    let x = x.unwrap();
    println!("Punkty:");
    for i in 0..xs.len() {
        println!("({}, {})", xs[i], ys[i]);
    }
    let result2 = lagrange::lagrange_interpolation(x, &xs, &ys);
    println!("f({}) -> {}", x, result2);
}

fn main() {
    lagrange_printable();
}