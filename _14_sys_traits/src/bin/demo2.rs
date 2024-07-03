
/// Copy trait
/// 和 Clone trait 不同的是， Copy Trait 没有任何额外的方法，
/// 它只是一个标记 trait，marker trait，
/// pub trait Copy: Clone {}
///
/// 从这个定义来看，要实现 Copy trait的话，必须实现 Clone trait，
/// 然后实现一个空的 Copy trait，
/// 这样的 trait 虽然没有任何行为，但它可以用作 trait bound 来进行类型安全检查 ，
/// 所以我们管它叫标记 trait, marker trait
///
/// 和 Clone 一样，如果数据结构的所有字段都实现了 Copy ，也可以用 #[derive(Copy)]
/// 宏来为数据结构实现 copy，
///
/// 代码会出错，因为 String 类型没有实现 copy，因此  developer 数据结构只能 clone,
/// 无法 copy，
/// 如果类型实现了 copy，在赋值，函数调用的时候，值会被拷贝，否则 所有权会被移动
///
/// Developer 类型在做参数传递时，会执行 move 语义，而 Language 会执行 Copy 语义
///
/// 在讲所有权可变 不可变引用的时候提到，不可变引用实现了 Copy，
/// 而可变引用 &mut T 没有实现 Copy，为什么是这样
///
/// 如果可变引用实现了 copy trait，那么生成一个可引用然后把赋值给另一个变量时，就会违背所有权规则 ，
/// 同一个作用域下只能有一个可变引用，
///

#[derive(Clone, Copy, Debug)]
struct Developer {
    // name: String,
    age: u8,
    lang: Language,
}

#[derive(Clone, Copy, Debug)]
enum Language {
    Rust,
    TypeScript,
    Cpp,
    Java,

}

fn main() {

}