use std::rc::Rc;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_rc() {
    /// 上面的代码创建了三个Rc，分别是 a b c ，它们共同指向堆上的相同数据，也就是说
    /// 堆上的数据有了三个共享的所有者
    let a = Rc::new(1);
    let _b = a.clone();
    let _c = a.clone();
    println!("{}", Rc::strong_count(&a));
}