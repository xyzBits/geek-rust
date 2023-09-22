use std::str::FromStr;

use regex::Regex;

pub trait Parse {
    fn parse(s: &str) -> Self;
}


impl<T> Parse for T
    where T: FromStr + Default
{
    fn parse(s: &str) -> Self {
        let regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        let default = || Default::default();

        if let Some(captures) = regex.captures(s) {
            captures
                .get(0)
                .map_or(default(),
                        |s|
                            s.as_str()
                                .parse()
                                .unwrap_or_else(|_| default()))
        } else {
            default()
        }
    }
}

fn main() {
    let result = u8::parse("234 hello world");
    println!("{}", result);

    assert_eq!(u32::parse("123abcd"), 123);
    assert_eq!(u32::parse("123.45abcd"), 0);
    assert_eq!(f64::parse("123.45abcd").to_string(), "123.45");
    assert_eq!(f64::parse("abcd").to_string(), "0");
}