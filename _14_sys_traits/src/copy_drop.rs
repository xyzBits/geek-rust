extern crate core;

use core::slice;
use std::fmt::{Debug, Formatter};

// 注意这里，我们实现了 copy，这是因为 *mut u8 size都支持 copy
#[derive(Clone, Copy)]
struct RawBuffer {
    // 裸指针是一种没有所有权或借用规则的指针，与引用不同，裸指针不受借用规则的约束，因此它们可以指向任何地方，可以是有效的，也可以是无效的
    // 裸指针用 *const / *mut u8 来表示，这和引用 & 不同
    ptr: *mut u8,
    len: usize,
}

impl From<Vec<u8>> for RawBuffer {
    fn from(vec: Vec<u8>) -> Self {
        let slice = vec.into_boxed_slice();

        Self {
            len: slice.len(),
            // into_raw 之后，Box就不管这块内存的释放了，RawBuffer 需要处理释放
            ptr: Box::into_raw(slice) as *mut u8,
        }
    }
}

/// 如果 RawBuffer 实现了 Drop trait，就可以在所在者退出时释放堆内存
/// 然后 Drop trait 会跟 Copy trait 冲突，要么不实现 Copy，要么不实现 Drop
/// 如果不实现Drop，就会导致 内存泄漏，但它不会对正确性有任何破坏
/// 比如不会出现use after free 这样的问题
///
// impl Drop for RawBuffer {
//     #[inline]
//     fn drop(&mut self) {
//         let data =
//             unsafe {
//                 Box::from_raw(slice::from_raw_parts_mut(self.ptr, self.len))
//             };
//         drop(data)
//     }
// }


impl AsRef<[u8]> for RawBuffer {
    fn as_ref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.ptr, self.len)
        }
    }
}

impl Debug for RawBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let data = self.as_ref();
        write!(f, "{:p}, {:?}", self.ptr, data)
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

    // 这里不用特意 drop ，写出来只是说明 copy 出来的 buf 被 drop 了
    drop(buf)
}