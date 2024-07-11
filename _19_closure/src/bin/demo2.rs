use std::collections::HashMap;
use std::mem::size_of_val;

/// 闭包的大小跟参数，局部变量都无关，只跟捕获的变量有关，
/// 函数调用，参数和局部变量都在栈中存放，
/// 调用时才在栈上产生的内存分配，
/// 说到底和闭包类型本身是无关的，
/// 所以闭包大小跟他们无关
///
/// 那一个闭包类型在内存中究竟是如何排布的，
/// 和结构体有什么不同
///
/// 回到刚才闭包和结构体的比较，在Rust 中，闭包产生的匿名数据类型，
/// 格式和struct 是一样的，闭包是存储在栈上的，并且除了捕获的数据外，
/// 闭包本身不包含任何额外函数指针指向闭包的代码
///
/// 在其他语言中，闭包变量因为多重引用导致生命周期不明确，但rust 从一开始就消灭了这个问题
///
/// 如果不使用move 转移所有权，闭包会引用上下文中的变量，这个引用受借用规则的约束
/// 所以只要编译通过，那么闭包变量的引用就不会超过变量的生命周期，没有内存安全问题
///
/// 如果使用move 转移所有权，上下文中的变量在转移秘有权后就无法访问，闭包完全
/// 接管这些变量，它们的生命周期和闭包一致，所以也不会有内存安全问题
///
/// Rust 为每个闭包生成一个新的类型，以使得调用闭包时可以直接和代码对应，省去了
/// 函数指针再转一道手的额外消耗
/// 

fn main() {
    // 长度为 0
    let c1 = || println!("hello world");

    // 和参数无关，长度也为 0
    let c2 = |i: i32| println!("hello {}", i);

    let name = String::from("tom");
    let name1 = name.clone();

    let mut table = HashMap::new();
    table.insert("hello", "world");

    // 如果捕获一个引用，长度为 8
    let c3 = || println!("hello: {}", name);

    // 捕获移动的数据  name1 长度为24，+ table 长度 48， closure 长度 72
    let c4 = move || println!("hello: {}, {:?}", name1, table);


    let name2 = name.clone();
    let c5 = move || {
        let x = 1;
        let name3 = String::from("jerry");
        println!("hello: {}, {:?}, {:?}", x, name2, name3);
    };

    println!(
        "c1: {}, c2: {}, c3: {}, c4: {}, c5: {}, main: {}",
        size_of_val(&c1),
        size_of_val(&c2),
        size_of_val(&c3),
        size_of_val(&c4),
        size_of_val(&c5),
        size_of_val(&main),
    )







}