
/// iter 是 lazy interface
/// 也就是说这段代码直到 collect 时才真正开始运行
/// 之前的部分只不过是在不断地生成新的结构
/// 来积累处理逻辑而已
///
/// Iterator 大部分方法都返回了一个实现了 Iterator 的数据结构
/// 所以可以这样一路链式下去
/// 在 Rust 标准库中，这些数据结构被称为 Iterator Adaptor
///
fn main() {
    let result = vec![1, 2, 3, 4]
        .iter()
        .map(|v| v * v)
        .filter(|v| *v > 4)
        .take(1)
        .collect::<Vec<_>>();

    println!("{:?}", result);
}