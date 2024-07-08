use std::borrow::Cow;

fn main() {

    let foo = "hello world";

    let mut bar = Cow::from(foo);

    // 这里并没有发生复制
    println!("{}", bar);

    // 这里发生了复制
    bar.to_mut().push_str(" Rust");

    println!("{}", bar);

    // 原来的字符串 foo 仍然可用，而且没有发生变化
    println!("{}", foo);

}