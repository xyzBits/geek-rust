///

fn main() {

    let mut x = 42;
    let y = &mut x;

    // 解引用，内部调用 DerefMut ，其实现就是 *self
    // *(y.deref()) = *y
    *y += 1;
}