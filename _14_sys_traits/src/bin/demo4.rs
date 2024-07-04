/// Sized trait 用于标记具有大小的类型
/// 在使用泛型参数时，Rust 编译器会自动为泛型参数加上 Sized 约束，
/// 比如下面的 Data<T> 和处理 Data<T> 的函数 process_data


// struct Data<T> {
//     inner: T,
// }


struct Data<T: ?Sized> {
    inner: T,
}
// fn process_data<T>(data: Data<T>) {
//     todo!()
// }

//Trait `std::marker::Sized` is not implemented for `Data<T>`
// [E0277] `Data<T>` does not have a constant size known at compile-time
fn process_data<T: Sized>(data: Data<T>) {
// fn process_data<T: ?Sized>(data: Data<T>) {
    todo!()
}

/// 大部分时候，我们希望能自动添加这样的约束，因为这样定义出的泛型结构，
/// 在编译期大小是固定的，可以作为参数传递给函数，如果没有这个约束，
/// T 是大小不固定的类型，process_data 函数会无法编译
///
/// 但是这个自动添加的约束有的时候不太适用，在少数情况下，需要 T 是可变类型，
/// 怎么办，Rust 提供了 ?Sized 来摆脱 个约束
///
/// 如果开发者显式定义了 T: ?Sized ，那么 T 就可以是任意大小，
/// [T] str 的大小是不固定的


fn main() {

}