// use jemalloctor::Jemalloc;
/// Box<T>
/// Box 是 Rust 中最基本的在堆上分配内存的方式，
/// 绝大多数其他包含堆内存的数据类型，内部都是通过 Box<T> 完成的，比如 Vec<T>
///
/// C 语言使用 malloc calloc realloc free 来处理内存的分配
/// 很多时候，被分配出来 的内存在函数调用中来来回回使用，导致谁应该负责释放这件事很难确定，
/// 给开发者造成了极大的心智负担
///
/// C++ 在此基础上改进了一下，提供了一个智能指针，unique_ptr，
/// 可以在指针退出作用域的时候释放堆内存，这样保证了堆内存的单一所有权，
/// 并唯一拥有这个指针
///
///
/// 在堆上分配内存，要使用 内存分配器，allocator
/// 操作系统是知道如何分配和管理堆内存的的
///
/// 设计内存分配器的目的除了保证正确性之外 ，
/// 就是为了有效地利用剩余内存，并控制内存在分配和释放过程中产生的碎片数量，
/// 在多核环境下，它还能够高效地处理并发请求，
///
/// 堆上分配内存的 Box<T> 其实还有一个缺省的泛型参数 A，
/// 就需要满足 Allocator trait 并且默认是 Global
/// pub struct Box<T: ?Sized, A: Allocator = Global>(Unique<T>, A)
///
/// Allocator trait 提供很多方法，
/// allocate 是主要方法，用于分配内存，对应 C 的 malloc calloc,
/// deallocate 用于释放内存，对应 C 的 free
/// 还有 grow shrink 用来扩大或缩小堆上已 分配的内存，对应 C 的 realloc
///
/// 如果你想替换默认的内存分配器，可以使用 #[global_allocator] 标记，定义你自己的全局分配器，
/// 下面的代码展示了如何 在 Rust 下使用 jemalloc
///
///

// static GLOBAL: Jemalloc = Jemalloc;
fn main() {
    let hello = String::from("hello");
    let len = hello.len();
    println!("{}", len);
}