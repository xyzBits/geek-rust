
/// 在遍历可变数组 data 的过程中，还往 data 里添加新数据，这是很危险的动作，
/// 因为它破坏了循环的不变性，容易导致死循环甚至系统崩溃，
/// 所以，同一个作用域下有多个可变引用，是不安全的
///
///
fn main() {
    let mut data = vec![1, 2, 3];

    for item in data.iter_mut() {// first mutable borrow occurs here
        // cannot borrow data as mutable more than once at a time
        data.push(*item + 1);
    }
}