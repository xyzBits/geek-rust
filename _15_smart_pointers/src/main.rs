
/*
数据结构中最容易让人困惑的就是智能指针
    指针是一个持有内存地址的值，可以通过解引用来访问它指向的内存地址，
    理论上可以解引用到做任意数据类型，引用是一个特殊的指针，它的解引用访问是受限的，
    只能解引用到它引用数据的类型，不能用作它用

    什么是智能指针呢？
        在指针和引用的基础上，Rust 偷师 c++，提供了智能指针，
        智能指针是一个表现行为很像指针的数据结构，但除了指向数据的指针外，它还有元数据以提供额外的处理能力

        智能指针一定是胖指针，但胖指针不一定是智能指针

        &str 是一个胖指针，它有指向堆内存字符串的指针，同时还有关于字符串长度的元数据

        String 除了多了一个 capacity 字段，似乎也没有什么特殊，但 String 对堆上的值
        有所有权，而 &str 是没有所有权的，这是 Rust 中智能指针和普通胖指针的区别


        和普通结构体不同的是，String 实现了 Deref DerefMut，这使得它在解引用时，
        会得到  &str

        再清晰一下定义，在 Rust中，凡是需要做资源回收的数据结构，且实现了
        Deref DerefMut Drop ，都是智能指针

 */
fn main() {
    println!("Hello, world!");

    let s = String::new();
    let ref_s = &s;
    // let result = *ref_s;
}
