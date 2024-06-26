use std::cell::RefCell;

/// Rc 是一个只读的引用计数器，你无法拿到 Rc 结构 内部数据的可变引用，来修改这个数据
///
/// 这里我们需要使用 RefCell
/// 和 Rc 类似，RefCell 也绕过了 Rust 编译器的静态检查，允许我们在运行时，
/// 对某个只读数据进行可变借用，内部可变性
///
/// 内部可变性，联想到外部可变性
///
/// let mut 显示地声明一个可变的值，或者，用 &mut 声明一个可变引用，
/// 编译器可以在编译时进行严格地检查，
/// 保证只有可变的值或者可变的引用，
/// 才能修改值内部的数据，这被称为外部可变性，
/// 外部可变性通过 mut 关键字声明
///
/// 这样不够灵活，有时候希望能够绕开这个编译时的检查，对并未声明成 mut 的值或者引用，
/// 也想进行修改，
/// 也就是说，
///
/// 在编译器的眼里，值是只读的，但是在运行时，这个值可以得到可变借用，从而修改内部的数据，
/// 这就是RefCell 的用武之地
///
/// data 是一个 RefCell，初始只有 1，data 并未声明为可变变量，
/// 之后我们可以通过 RefCell 的 borrow_mut() 方法，来获得一个可变的内部引用，
/// 然后向其添加元素，然后可以通过 RefCell 的 borrow() 方法，获得一个不可变的内部引用
///
/// 为什么要把获取和操作可变借用的代码，用花括号封装到一个作用域下，
///
/// 根据所有权规则，在同一个作用域下，不能同时拥有活跃的可变借用和不可变借用，通过这对花括号，
/// 明确的缩小了可变借用的生命周期，不至于和后续的不可变借用冲突
///
fn main() {

    let data = RefCell::new(vec![0]);
    {
        // 通过 RefCell 获得 内部数据的可变借用
        let mut v = data.borrow_mut();
        (1..5).for_each(|item| (*v).push(item));

    }

    println!("data: {:?}", data.borrow());

}