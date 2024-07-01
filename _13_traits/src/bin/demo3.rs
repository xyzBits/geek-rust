use std::str::FromStr;
use std::{f64, u32};
use regex::Regex;

pub trait Parse {
    fn parse(s: &str) -> Self;
}

/// 通过对 带有约束的泛型参数实现 trait，一份代码就实现了 u32 f64 等类型的 parse trait
/// 非常精简，不过，无法正确解析字符串时，我们返回了默认值，难道不是应该返回一个错误么
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

    }
}

fn main() {
    assert_eq!(u32::parse("123abcd"), 123);
    assert_eq!(u32::parse("123.45abcd"), 0);
    assert_eq!(f64::parse("123.45abcd"), 123.45);
    assert_eq!(f64::parse("abcd"), 0f64);
}