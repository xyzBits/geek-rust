extern crate core;

use core::slice;
use std::fmt::{Debug, Formatter};

/// Drop trait
///
/// 大部分场景无需为数据结构提供 Drop trait
/// 系统默认会依次对数据结构的每个域做 drop
/// 有两种情况你可能需要手工实现 drop
///
/// 第一种是希望在数据结束生命周期的时候做一些事情，比如记录日志
/// 第二种是需要对资源回收的场景，编译器并不知道你额外使用了哪些资源，也就无法帮助你drop 它们
///     比如说锁资源的释放，在 MutexGuard<T> 中实现了 Drop 来释放资源
///
/// 需要注意的是，Copy trait 和 Drop trait 是互斥的，两者不能共存，当你尝试为同一种数据
/// 类型实现 copy时，也实现 Drop，编译器就会报错，
/// 这其实很好理解 ，Copy 是按位做浅拷贝，那么它会默认拷贝的数据没有需要释放的资源，
/// 而 Drop 恰恰是为了释放额外的资源而生的
///
/// 代码辅助理解 ，在代码中，强行用 Box::into_raw 获得堆内存的指针
/// 放入 RawBuffer 结构中，这样就接管了这块堆内存的释放
///
/// 虽然 RawBuffer 可以实现 Copy trait，但这样一来就无法实现 Drop trait，
/// 如果程序非要这么写，会导致优点泄漏，因为该释放的堆内存没有释放
///
/// 但是这个操作不会破坏 rust 的正确性，即便你 Copy 了 N 份 RawBuffer，
/// 由于无法实现 Drop trait，rawBuffer 指向的那同一块内存堆内存不会释放，所以不会出现 use after free
/// 的内存安全问题

// 注意这里，我们实现了 copy trait，这是因为 *mut u8 / usize 都支持 Copy
#[derive(Clone, Copy)]
struct RawBuffer {
    /// 祼指针引用 *const *mut 来表述 ，这和引用的 & 不同
    ptr: *mut u8,
    len: usize,
}


impl From<Vec<u8>> for RawBuffer {
    fn from(value: Vec<u8>) -> Self {
        let slice = value.into_boxed_slice();
        Self {
            len: slice.len(),
            // into_raw 之后，Box 就不管这块内存的释放了，RawBuffer 需要处理释放
            ptr: Box::into_raw(slice) as *mut u8,
        }
    }
}


/// 如果 RawBuffer 实现了 Drop trait，就可以在所有者退出时释放堆内存
/// 然后，Drop trait 会跟 Copy trait 冲突，要么不实现 Copy，要么不实现 Drop
/// 如果不实现 Drop ，那么就会导致内存泄漏，但它不会对正确性有任何破坏
/// 比如不会出现 use after free 这样的问题
///
/// the trait Copy cannot be implemented for this type,
/// the type has a destructor
// impl Drop for RawBuffer {
//     #[inline]
//     fn drop(&mut self) {
//         todo!()
//     }
// }

impl Debug for RawBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let data = self.as_ref();
        write!(f, "{:p}: {:?}", self.ptr, data)
    }
}

impl AsRef<[u8]> for RawBuffer {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.ptr, self.len)
        }
    }
}

fn main() {
    let data = vec![1, 2, 3, 4];

    let buf: RawBuffer = data.into();

    use_buffer(buf);

    println!("buf: {:?}", buf);
}

fn use_buffer(buf: RawBuffer) {
    println!("buf to die: {:?}", buf);

    // 这里不用特意drop，写出来 只是为了说明 copy 出来 的 buf 被  drop 掉了

    drop(buf);
}