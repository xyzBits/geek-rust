use std::cell::RefCell;

/// 编译是没有任何问题的，
/// 如果运行，就会得到
/// already mutably borrowed: BorrowError
/// 所有权的借用规则在此依然有效，只不过它在运行时检测
///
/// 这就是外部可变可变性和内部可变性的区别
///
///                 使用方法                  所有权检查
/// 外部可变性        let mut 或者 &mut        编译时，如果不符合规则，产生编译错误
/// 内部可变性        使用 Cell 或者 RefCell     运行时，如果不符合规则，产生panic
fn main() {
    let data = RefCell::new(vec![0]);

    let mut v = data.borrow_mut();
    (1..44).for_each(|item| (*v).push(item));

    println!("data {:?}", data.borrow());
}