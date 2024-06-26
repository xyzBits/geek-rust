fn main() {
    let data = vec![10, 42, 3, 22, 12, 9, 4];
    let v = 34;
    let index_option = find_pos(data, v);

    if let Some(index) = index_option {
        println!("{}", index);
    } else {
        println!("nothing");
    }
}

/// 动态数组因为大小在编译期间无法确定，所以放在堆上，并且在栈上有一个包含了长度和容量的胖指针指向推上的内存
/// 每次将data作为参数传递一次，堆上的内存就会多一次引用
/// Rust 的规则
/// 1. 一个值只能被一个变量所拥有，这个变量被称为所有者
/// each value in Rust has a variable that's called its owner.
/// 2. 一个值同一时刻只有有一个所有者，也就是说不能有两个变量拥有相同的值，
/// 所以对应刚才所说的变量赋值，参数传递，函数返回等行为，旧的所有者会把值的所有权转移给新的所有者，
/// 以保证单一所有者的约束
/// 3. 当所有者离开作用域，其拥有的值 被丢弃，内存得到释放
/// 第三条规则中的作用域是一个新概念，它指一个代码块，在Rust中，一对花括号括起的代码区就是一个作用域，
/// 如果一个变量被定义在if{} 内，那么if语句结束 ，这个变量的作用域就结束了，其值会被丢弃，
/// 同样的，函数里定义的变量，在离开函数时会被丢弃
///
/// 在这段代码中，先创建了一个不可变数据  data，然后将data赋值给data1，按照所有权的规则 ，
/// 赋值之后，data指向的值被移动给了data1，它自己便不可访问了，
/// 而随后，data1作为参数传递给函数sum，在main函数下，data1 也不可访问了
/// 编译器会告诉我们，不能使用已经移动过的变量
///
/// 如果我们要在把data1传给sum，同时，还想让main能够访问 data，该怎么办，可以调用 data.clone()
/// 把data复制一份出来给data1，这样，在堆上就有 vec![1, 2, 3, 4] 两个互不影响可以独立释放的副本
///
/// 所有权规则，解决了谁真正拥有数据的生杀大权问题，让堆上数据的多重引用不复存在，这是它的最大优势
///
/// 但是这也会让代码复杂，尤其是一些只存储于栈上的简单数据，如果要避免所有权转移之后不能访问的情况，
/// 我们需要手动复制，会非常麻烦，效率也不高
///
/// Rust 考虑到了这一点，提供了两种方案
/// 1. 如果你不希望值的所有权被转移，在 Move 语义之外，Rust 提供了 Copy语义，如果一个数据结构实现了
/// Copy trait ，那么它就会使用 Copy 语义，这样，在你赋值或者传参的时候，值会自动按位拷贝
/// 2. 如果你不希望值的所有权被转移，又无法使用copy语义，那你可以借用数据，
fn find_pos(data: Vec<u32>, v: u32) -> Option<usize> {
    for (index, item) in data.iter().enumerate() {
        if *item == v {
            return Some(index);
        }
    }

    None
}