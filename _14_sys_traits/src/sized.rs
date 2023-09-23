use std::fmt::Debug;

// Sized trait 用于标记具有大小的类型，在使用泛型参数时，rust会自动为泛型参数加上Sized约束
//
struct Data<T> {
    inner: T,
}



fn process_data<T>(data: Data<T>) {

}

// 等价于下面的代码
// 因为定义出这样的泛型结构，在编译期，大小是固定的，可以作为参数传递给函数
// 如果没有这个约束，T 是大小不固定的类型 process_data 函数无法编译
#[allow(dead_code)]
struct DataV1<T: Sized> {
    inner: T,
}
fn process_data_v1<T: Sized>(data: Data<T>) {
    todo!()
}


#[allow(dead_code)]
struct UnsizedData<T: ?Sized> {
    inner: T,
}

// 无法编译通过，函数的参数必须是编译时确定大小 的
// fn process_unsized_data<T>(data: UnsizedData<T>)
//     where T: ?Sized {
//     todo!()
// }


fn main() {
    let data = Data {
        inner: "hello",
    };
    process_data(data);

    let x = 5;
    debug_trait(&x);

    let s = "hello";
    debug_trait(s);
}

/*
0 | fn debug_trait<T: Debug>(value: &T) {
   |                ^ required by this bound in `debug_trait`
help: consider relaxing the implicit `Sized` restriction
   |
40 | fn debug_trait<T: Debug + ?Sized>(value: &T) {
   |                         ++++++++
 */
// fn debug_trait<T: Debug>(value: &T) {
fn debug_trait<T: Debug + ?Sized>(value: &T) {
    println!("{:?}", value);
}