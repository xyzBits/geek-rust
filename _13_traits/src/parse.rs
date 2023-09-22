use regex::Regex;

pub trait Parse {
    fn parse(s: &str) -> Self;
}

impl Parse for u8 {
    fn parse(s: &str) -> Self {
        let regex: Regex = Regex::new(r"^[0-9]+").unwrap();
        if let Some(captures) = regex.captures(s) {
            captures
                .get(0)
                .map_or(0, |s| s.as_str().parse().unwrap_or(0))
        } else {
            0
        }
    }
}

fn main() {
    assert_eq!(u8::parse("123abcd"), 123u8);
    assert_eq!(u8::parse("1234abcd"), 0);
    assert_eq!(u8::parse("abcd"), 0);

    println!("{}", u8::parse("245 hello world"));
}