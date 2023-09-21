use std::fmt::format;

fn id<T>(x: T) -> T {
    x
}
fn main() {
    let int = id(42);
    let string = id("hello world");

    println!("{}", format!("{int}, {string}"));
}