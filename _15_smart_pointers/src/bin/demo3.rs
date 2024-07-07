use std::alloc::{GlobalAlloc, Layout, System};


/// 实现一个自己的内在分配器，来debug 一下，看看内在如何分配和释放，并不会
/// 实现某个分配算法
///
/// 首先看内存的分配，这里 MyAllocator 就用 System Allocator ，
/// 然后加 eprintln 和我们常用的 println 不同的时，
/// eprintln 将数据打印到 stderr
///

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let data = System.alloc(layout);
        eprintln!("ALLOC: {:p}, size {}", data, layout.size());
        data
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        eprintln!("FREE: {:p}, size {}", ptr, layout.size());
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;


#[allow(dead_code)]
struct Matrix {
    // 使用不规则的数字如 505 可以让 dbg! 的打印很容分辨出来
    data: [u8; 505],
}

impl Default for Matrix {
    fn default() -> Self {
        Self {
            data: [0; 505]
        }
    }
}
fn main() {
    // 在这句执行之前已经有好多内存分配
    let data = Box::new(Matrix::default());

    println!(
        "!!! allocated memory: {:p}, len: {}",
        &*data,
        std::mem::size_of::<Matrix>()
    );

    // data 在这里 drop ，可以在打印中看到 free
    // 之后还有很多其他的内存被释放

}