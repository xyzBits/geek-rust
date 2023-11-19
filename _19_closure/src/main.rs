use std::collections::HashMap;
use std::mem::size_of_val;
use std::thread;

fn main() {
    let s = "hello world".to_string();

    // let func: fn() = move || {
    //     println!("{}", s);
    // };
    //
    // let handle = thread::spawn(func);
    //
    // handle.join().unwrap();

    println!("Hello, world!");


    println!("==============================================");
    let c1 = || println!("hello world!");
    let c2 = |i: i32| println!("hello: {}", i);

    let name = String::from("dong fang");
    let name1 = name.clone();

    let mut table = HashMap::new();
    table.insert("hello", "world");

    let c3 = || println!("hello: {}", name);

    let c4 = move || println!("hello: {}, {:?}", name1, table);

    let name2 = name.clone();

    let c5 = move || {
        let x = 1;
        let name3 = String::from("world");
        println!("hello: {}, {:?}, {:?}", x, name2, name3);
    };

    // 闭包的大小跟参数，局部变量都无关，只跟捕获的变量有关
    println!("c1 = {}", size_of_val(&c1));
    println!("c2 = {}", size_of_val(&c2));
    println!("c3 = {}", size_of_val(&c3));
    println!("c4 = {}", size_of_val(&c4));
    println!("c5 = {}", size_of_val(&c5));

    // 在声明闭包的时候，我们并不需要指定闭包要满足的约束，但是当闭包作为函数
    // 的参数或者数据结构的一个域时，我们需要告诉调用者，对闭包的约束，
    // 还以thread::spawn 为例，它要求传入的闭包满足FnOnce trait
    
}
