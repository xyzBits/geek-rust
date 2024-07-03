
/// 用好 trait ，会让代码结构更加清晰，阅读和使用都更加符合 rust 生态的习惯
/// 比如数据结构实现了 Debug trait ，当想打印数据结构时，就可以用 {:?} 来打印
/// 如果实现了 From<T> ，可以直接使用 into() 方法来做数据转换
///
///
/// - Clone / Copy trait 约定了数据被深拷贝和浅拷贝的行为
/// - Read / Write trait 约定了 I/O 读写的行为
/// - Iterator 约定了迭代器的行为
/// - Debug 约定了数据如何被以 debug 的方式显示出来
/// - Default 约定数据类型的默认值如何产生的行为
/// From<T> / TryFrom<T> 约定了数据间如何转换的行为
///
/// 再学会几类比较重要的 trait，包括和内存分配释放相关的 trait，
/// 用于区别不同类型协助编译器做类型安全检查的标记 trait
/// 进行类型转换的 trait
/// 操作符相关的 trait
/// 以及 debug display default
///
///
/// 内存相关： Clone / Copy / Drop
///
/// Clone trait 有两个方法，clone() clone_from() ，后者有默认的实现，所以
/// 平时我们只需要实现 clone() 方法即可，
/// clone_from() 的作用，a.clone_from(&b) 和 a = b.clone()
///
/// 其实不是，如果 a 已经存在，在 clone 过程中会分配内存，那么用 a.clone_from(&b)
/// 可以避免内存分配，提高效率
///
/// Clone trait 可以通过派生宏直接实现，这样能简化不少代码，
/// 如果在你的数据结构中，每一个字段都已经实现了 Clone trait，你可以用 #[derive(Clone)]
///
/// 如果没有为 Language 实现 Clone 的话，Developer 的派生宏 Clone 将会编译出错，
/// 运行代码可以看到，对于 name，String 类型是 clone，其堆上的内存也被  Clone了一份，
/// 所以 Clone 是深度拷贝，栈内存和堆内存一起拷贝
///
/// clone 方法的接口是 &self，在绝大多数场合下都是适用的，
/// 在 clone 一个数据时，只需要已有数据的只读引用，
/// 但对 Rc<T> 这样在clone 时维护引用计数的数据结构，
/// clone 过程中会改变自己，所以要用 Cell<T> 这样提供内部可变性的结构来进行改变

#[derive(Clone, Debug)]
struct Developer {
    name: String,
    age: u8,
    lang: Language,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Language {
    Rust,
    TypeScript,
    Cpp,
    Haskell,
}

fn main() {
    let dev = Developer {
        name: "tom".to_string(),
        age: 18,
        lang: Language::Rust
    };

    let dev1 = dev.clone();

    println!("dev: {:?}, addr of dev name: {:p}", dev, dev.name.as_str());
    println!("dev: {:?}, addr of dev name: {:p}", dev1, dev1.name.as_str());

}