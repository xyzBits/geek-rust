use std::pin::Pin;

#[derive(Debug)]
struct Foo {
    x: i32,
    y: i32,
}

impl Foo {
    fn new() -> Self {
        Foo {
            x: 0,
            y: 1,
        }
    }
}

fn main() {
    // 先创建数据在堆上的 Box<Foo> 指针，然后基于 Box<Foo> 创建 Pin 指针
    let box_foo = Box::new(Foo::new());

    let pin_foo = Pin::new(box_foo);

    let foo_ref = &*pin_foo;

    println!("{:?}", foo_ref);


}




















