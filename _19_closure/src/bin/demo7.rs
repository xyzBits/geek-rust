use std::ops::Mul;

// 函数参数中使用闭包，是非常典型的用法，
// 另外闭包也可以作为函数的返回值

/// Rust 闭包的效率非常高，闭包捕获的变量，都储存在栈上，
/// 没有堆内存分配，其次因为闭包在创建时会隐式地创建自己的类型，
/// 每个闭包都是一个新的类型，通过闭包自己唯一的类型，
/// Rust 不需要额外的函数指针来运行闭包，
/// 所以闭包的调用效率和函数的调用几乎一致
///
/// FnOnce 只允许调用一次
/// FnMut 允许在执行时修改闭包捕获的数据，可以执行多次
/// Fn 不允许修改捕获的数据，可以执行多次
///
fn main() {
    let c1 = curry(5);
    println!("5 x 2 = {}", c1(2));

    let adder = curry(3.14);

    println!("pi x 4 x 4 = {}", adder(4. * 4.));
}

fn curry<T>(x: T) -> impl Fn(T) -> T
where
    T: Mul<Output=T> + Copy,
{
    move |y| { x * y }
}