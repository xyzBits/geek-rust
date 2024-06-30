use std::str::FromStr;
use std::{f64, u32};
use regex::Regex;

pub trait Parse {
    fn parse(s: &str) -> Self;
}

impl<T> Parse for T
where
    T: FromStr + Default,
{
    fn parse(s: &str) -> Self {
        let re = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();

        // 生成一个创建默认的闭包，这里主要是为了简化后续的代码，
        // Default::default() 返回的类型根据上下文能推导出来 ，是 Self
        // 而我们约定了 Self，也就是 T 需要实现 Default  trait
        let default = || Default::default();

        if let Some(captures) = re.captures(s) {
            captures
                .get(0)
                .map_or(default(), |s| s.as_str().parse().unwrap_or(default()))
        } else {
            default()
        }

        todo!()
    }
}

fn main() {
    assert_eq!(u32::parse("123abcd"), 123);
    assert_eq!(u32::parse("123.45abcd"), 0);
    assert_eq!(f64::parse("123.45abcd"), 123.45);
    assert_eq!(f64::parse("abcd"), 0f64);
}