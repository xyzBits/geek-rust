fn main() {}

/// first 函数只接受一个字符串引用，找到其中的第一个单词并返回
/// 虽然我们没有做任何的生命周期标注，但编译器会通过一些简单的规则为函数自动添加标注
///
/// 1. 所有引用类型的参数都有独立的生命周期 'a 'b 等
/// 2. 如果只有一个引用型输入，它的生命周期会赋给所有输出
/// 3. 如果有多个引用类型的参数，其中一个是 self，那么它的生命周期会赋给所有输出
///
/// 规则  3 适用于 trait 或者自定义数据类型，
/// 必须要能从 input 的生命周期标注上 得到 output 的生命周期，
/// 否则，编译器无法编译
///
fn first<'a>(s: &'a str) -> &'a str {
    let trimmed = s.trim();
    match trimmed.find(' ') {
        None => "",
        Some(pos) => &trimmed[..pos]
    }
}