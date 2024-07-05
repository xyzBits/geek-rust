use std::ops::{Deref, DerefMut};

/// 在 Rust 里，绝大多数 智能指针都实现了 Deref
/// 我们也可以为自己的数据结构实现 Deref
#[derive(Debug)]
struct Buffer<T> (Vec<T>);

impl<T> Buffer<T> {
    pub fn new(v: impl Into<Vec<T>>) -> Self {
        Self(v.into())
    }
}

impl<T> Deref for Buffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Buffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
/// 在这个例子中，数据结构  Buffer<T> 包裹住了 Vec<T>
/// 但这样一来，原本 Vec<T> 实现了的很多方法，现在使用起来很不方便，
/// 需要用 buf.0 来访问，怎么办
///
/// 可以实现 Deref DerefMut ，这样在解引用的时候，直接访问 buf.0
/// 省去了 代码的 啰嗦和数据结构内部字段的隐藏
///
/// 在这段代码中，写 buf.sort() 的时候，并没有做解引用的操作，
/// 为什么相当于访问了 buf.0.sort()呢，
/// 这是因为 sort() 方法的第一个参数是 &mut self,
/// 此时 Rust 编译器会强制做 Deref DerefMut 的解引用，
/// 所以这相当于 (*(&mut buf)).sort()

fn main() {
    let mut buf = Buffer::new([1, -1, 9, 2, 3, 4]);

    // 因为实现了 Deref DerefMut，这里 buf 可以直接访问 Vec<T> 的方法
    // 下面这句，相当于 (&mut buf).deref_mut().sort()
    // 也就是 (&mut buf.0).sort()
    buf.sort();
    println!("{:?}", buf);
}