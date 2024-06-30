use regex::Regex;

/// 假设要做一个字符串解析器，可以把字符串的某部分解析成某个类型，
/// 那么可以定义这个 trait ，它有一个方法 是 parse，这个方法可以接受一个字符串引用，返回 self
/// 这个 parse 方法是 trait 的静态方法，因为它的第一个参数和 self 无关，所以在调用时需要使用
/// T::parse(str)
/// 我们为 u8 这个数据结构来实现 parse，123abc 解析出整数 123
/// abcd 解析出 0


pub trait MyParse {
    fn parse(s: &str) -> Self;
}


impl MyParse for u8 {
    fn parse(s: &str) -> Self {
        let re: Regex = Regex::new(r"^[0-9]+").unwrap();
        if let Some(captures) = re.captures(s) {
            captures
                .get(0)
                .map_or(0, |s| s.as_str().parse().unwrap_or(0))
        } else {
            0
        }
    }
}

fn main() {
    let result = u8::parse("123abc");
    println!("{}", result);
}