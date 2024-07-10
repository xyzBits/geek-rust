use std::ops::Deref;
/// 当我们需要在堆上创建固定大小的数据集合，且不希望自动增长，
/// 那么，可以先创建 Vec<T> ，再转换成 Box<T>
fn main() {
    let mut v1 = vec![1, 2, 3, 4];
    v1.push(5);

    println!("{}", v1.capacity());

    // 从 Vec<T> 转换成 Box<[T]> ，此时会丢弃多余的 capacity
    let b1 = v1.into_boxed_slice();
    let mut b2 = b1.clone();

    let v2 = b1.into_vec();
    println!("{}", v2.capacity());

    assert!(b2.deref() == v2);

    b2[0] = 33;

    println!("{:?}", b2);

}