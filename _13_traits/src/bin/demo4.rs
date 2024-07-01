use std::str::FromStr;
use regex::Regex;

/// 如果返回默认值的话，会和解析 0abcd 这样的情况混淆，不知道解析出的0，
/// 究竟是出错了，还是本该解析出0
///
/// 所以更好的方式是 parse 函数返回一个 Result<T, E>
/// 但是这里的 E 让人犯了难，要返回的错误信息，在定义 trait 时并不确定，
/// 不同的实现者可能有不同的的错误类型，这里 trait 的定义者最好能够把这种灵活性留给
/// trait 的实现者，该如何处理
///
/// 想想 trait 既然允许 trait 内部包含方法，也就是关联函数，可不可以进一步包含关联类型呢，
/// 答案是肯定的
///
/// 带关联类型的 trait
/// Rust 允许 trait 内部包含关联类型，实现时跟关联函数一样，它也需要实现关联类型，
/// 我们看 Rust 怎么为 parse trait 添加关联类型


// trait Parse {
//     fn parse(s: &str) -> Result<Self, E>;
// }

/// 有了关联类型 Error，parse trait 就可以在出错时返回合理的错误信息，
trait Parse {
    type Error;
    fn parse(s: &str) -> Result<Self, Self::Error>
    where Self: Sized;
}

/// 允许用户把错误类型延迟到 trait 实现时才决定，这种带有关联类型的 trait 比普通 trait，更加灵活，抽象度更高
/// trait 方法里的参数，或者返回值，都可以用关联类型来表述，而在实现有关联类型的 trait时，
/// 只需要额外提供关联类型的具体类型即可 
impl <T> Parse for T
where T: FromStr + Default {
    type Error = String;

    fn parse(s: &str) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let re = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();


        if let Some(captures) = re.captures(s) {

            captures
                .get(0)
                .map_or(Err("failed to capture".to_string()),
                |s| s.as_str()
                    .parse()
                    .map_err(|_err| "failed to parse captured string".to_string()))

        } else {
            Err("failed to parse string".to_string())
        }

    }
}

fn main() {

    assert_eq!(u32::parse("123abcd"), Ok(123));

}

#[test]
fn test_map_or() {
    // returns the provided default result (if none), or applies a function to the contained value (if any)

    let x = Some("foo");
    assert_eq!(x.map_or(42, |v| v.len()), 3);

}