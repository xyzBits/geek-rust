fn main() {
    let r = local_ref();
    println!("{:p}", r);
}

#[allow(unused_variables)]
fn local_ref<'a>() -> &'a i32 {
    let a = 42;

    //不能返回对局部变量a 的引用

    // returns a reference to data owned by the current function
    // &a
    todo!()
}