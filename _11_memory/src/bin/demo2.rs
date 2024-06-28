use std::collections::HashMap;
use std::mem::size_of;

/// Enum 是 Rust 下的一个标签联合体，tagged union，它的大小是标签的大小，加上最大类型的长度
/// Option<T> Result<T, E>
/// Option 是有值 无值这种最简单的枚举类型，
/// Result 包含成功返回数据和错误返回数据的枚举类型，
///
/// 三条对齐规则 ，tag 后的内存，会根据其大小进行对刘，
/// 所以对于 Option<u8> 其长度是 1 + 1 = 2 字节，
/// 而 Option<u64> 其长度是 8 + 8 = 16 字节，
/// 一般而言，64位 CPU下，enum 最大长度是 最大类型的长度 + 8
/// 因为 64 位 CPU 的最大对齐是 64bit，也就是8 byte
///Type                        T    Option<T>    Result<T, io::Error>
// ----------------------------------------------------------------
// u8                          1        2           16
// f64                         8       16           16
// &u8                         8        8           16
// Box<u8>                     8        8           16
// &[u8]                      16       16           16
// String                     24       24           24
// Vec<u8>                    24       24           24
// HashMap<String, String>    48       48           48
// E                          56       56           56
/// Option 配合带有引用类型的数据结构，比如 &u8 Box Vec HashMap 没有额外占用空间，
/// 对于 Option 结构而言，它的 tag 只有两种情况 0 1，tag 为0时，表示  None，tag 为 1 时，表示 Some
/// 
///


enum E {
    A(f64),
    B(HashMap<String, String>),
    C(Result<Vec<u8>, String>),

}

// 这是一个声明宏，它会打印各种数据结构本身的大小，在 Option 中的大小，以及在 Result 中的大小
macro_rules! show_size {
    (header) => {
        println!(
            "{:<24} {:>4}    {}    {}",
            "Type", "T", "Option<T>", "Result<T, io::Error>"
        );
        println!("{}", "-".repeat(64));
    };
    ($t:ty) => {
        println!(
            "{:<24} {:4} {:8} {:12}",
            stringify!($t),
            size_of::<$t>(),
            size_of::<Option<$t>>(),
            size_of::<Result<$t, std::io::Error>>(),
        )
    };
}

fn main() {

    show_size!(header);
    show_size!(u8);
    show_size!(f64);
    show_size!(&u8);
    show_size!(Box<u8>);
    show_size!(&[u8]);
    show_size!(String);
    show_size!(Vec<u8>);
    show_size!(HashMap<String, String>);
    show_size!(E);
}