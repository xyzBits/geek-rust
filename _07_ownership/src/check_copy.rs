fn main() {
    types_impl_copy_trait();
}

/// 原生类型，包括函数，不可变引用和祼指针都实现了 copy
/// 数组和元组，如果其内部的数据结构实现了copy，那么它们也实现了copy
/// 可变引用没有实现copy
/// 非固定大小的数据结构，没有实现copy
///
/// 所有权：一个值只能被一个变量所拥有，且同一时刻只能有一个所有者，当所有者离开作用域，其拥有的值被丢弃，内存得到释放
/// move 语义：赋值或者传参会导致值move，所有权被转移，一旦所有权被转移，之前的变量就不能访问
/// copy 语义：如果值实现了copy trait，那么赋值或传参就会使用copy 语义，相应的值会被按位拷贝，产生新的值
fn is_copy<T: Copy>() {}

fn types_impl_copy_trait() {
    is_copy::<bool>();
    is_copy::<char>();

    is_copy::<u8>();
    is_copy::<u16>();
    is_copy::<u32>();
    is_copy::<u64>();
    is_copy::<usize>();

    is_copy::<f32>();
    is_copy::<f64>();

    // 函数指针是 copy
    is_copy::<fn()>();

    is_copy::<*const String>();
    is_copy::<*mut String>();

    // 不可变引用是 copy
    is_copy::<&[Vec<u8>]>();
    is_copy::<&String>();

    // 数组，元组，如果其内部类型是 copy ，那么它们也是 copy
    is_copy::<[u8; 4]>();
    is_copy::<(&str, &str)>();
}

fn types_not_impl_copy_trait() {
    // DST 类型不是 copy
    // is_copy::<str>();
    // is_copy::<[u8]>();

    // is_copy::<Vec<u8>>();
    // is_copy::<String>();

    // is_copy::<&mut String>();

    // is_copy::<[Vec<u8>; 4]>();
    // is_copy::<(String, u32)>();


}


/// Rust 中，分配堆上的数据结构可以引用栈上的数据么，为什么
/// 可以，
/// 只要栈上的数据生命周期大于堆上数据的生命周期，就可以引用
/// 堆上数据被回收之前，栈上的数据一定会存在的情况下，是可以的
///
/// x ---> a 表示 指针 x 指向数据  a
/// 在其他语言中，内存中可以出现如下的情况， x --> a; y --> a; z --> a
/// 但是在 Rust中，假设最初为 x --> a，当我们接下来需要 y --> a时，我们可以认为 x 不会被使用了，
/// 也就是 x --> a
#[test]
fn test_heap_ref_stack_data() {
    let x = 1;
    let y = 2;
    let data = vec![x, y];
    println!("{:?}", data);
}