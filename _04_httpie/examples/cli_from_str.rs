use std::str::FromStr;

/// parse a value from a string
/// FromStr's from_str method is often used implicityly
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    // (1, 2)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.split_once(','))
            .ok_or(ParsePointError)?;

        let x_from_str = x.trim().parse::<i32>().map_err(|_| ParsePointError)?;
        let y_from_str = y.trim().parse::<i32>().map_err(|_| ParsePointError)?;

        Ok(Point { x: x_from_str, y: y_from_str })
    }
}

fn main() {
    let expected = Ok(Point { x: 1, y: 2 });

    let result = "(1, 2)".parse::<Point>();
    println!("{:?}", result);
    assert_eq!(result, expected);
}
