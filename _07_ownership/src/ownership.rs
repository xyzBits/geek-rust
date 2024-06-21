fn main() {
    let data = vec![10, 42, 3, 22, 12, 9, 4];
    let v = 34;
    let index_option = find_pos(data, v);

    if let Some(index) = index_option {
        println!("{}", index);
    } else {
        println!("nothing");
    }
}

/// 动态数组因为大小在编译期间无法确定，所以放在堆上，并且在栈上有一个包含了长度和容量的胖指针指向推上的内存
/// 每次将data作为参数传递一次，堆上的内存就会多一次引用
/// Rust 的规则
fn find_pos(data: Vec<u32>, v: u32) -> Option<usize> {
    for (index, item) in data.iter().enumerate() {
        if *item == v {
            return Some(index);
        }
    }

    None
}