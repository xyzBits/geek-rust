fn main() {
    let result = vec![1, 2, 3, 4]
        // 这里 Vec<T> 在调用 iter() 时被解引用成 &[T] ，所以可以访问  iter()
        .iter()
        .map(|v| v * v)
        .filter(|v| *v < 16)
        // .take(1)
        // iterator 大部分方法都返回了一个实现了 Iterator的数据结构，所以可以这样一路链式下去
        .collect::<Vec<_>>();// 迭代器是一个 lazy interface，这段代码直到运行到collect 时才真正开始执行，之前的部分只不过是在生成 新的结构
    println!("{:?}", result);
}