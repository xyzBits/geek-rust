use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

/// 为什么标准库的数据结构 如 Rc Vec 用那么多 unsafe，但别人告诉我，unsafe 不好
/// C 语言的开发者也认为 asm 不好，但 C 的很多库里也大量使用 asm
/// 标准库的责任是，在保证安全的情况下，即使牺牲一定的可读性，也要用最高效的手段来实现
/// 要实现的功能，同时，为标准库提供一个优雅，高级的抽象 ，让他们可以在绝大多数场合下
/// 写出漂亮的代码，无需和丑陋打交道
///
/// Rust 中，unsafe 代码把程序的正确性和安全性交给开发者来保证，而标准库的开发者花
/// 了大量的精力和测试来保证这种安全性和正确性，
/// 而我们自己写 unsafe 时，除非有经验丰富的开发者 review 代码，否则，有可能疏于骊并发情况的考虑，
/// 写出有问题的代码
///
/// 所以，只要不是必须的，建议不要写 unsafe 代码，毕竟大部分我们要处理的问题，都可以通过良好的
/// 设计，合适的数据结构和算法来实现
///
///
/// Rust 中，如何声明全局变量呢
/// const static 它们都可以用于声明全局变量，
/// 但注意，除非使用 unsafe, static 无法作为 mut 使用，因为这意味着它可能在多个线程下被修改，
/// 所以不安全
///
/// 如果 你的确想用全局变量，可以用 Mutex<T> ，然后，初始化它很麻烦，不过，你可以用一个库，
/// lazy_static
///

lazy_static! {
    static ref HASHMAP: Arc<Mutex<HashMap<u32, &'static str>>> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        Arc::new(Mutex::new(m))
    };
}

fn main() {

    let mut map = HASHMAP.lock().unwrap();

    map.insert(3, "waz");
    println!("{:?}", map);
}