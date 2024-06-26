
/// 在这段代码里，不可变数组 data1 引用了可变数组 data 中的一个元素，
/// 这是个只读引用，
/// 后续我们往 data 中添加了 100 个元素，在调用 data.push() 时，
/// 我们访问了 data 的可变引用
///
/// 这段代码中， data 的只读引用和可变引用共存，似乎没什么影响，因为 data1 引用的元素没有任何改动
///
/// 内存不安全操作：
///     如果继续添加元素，堆上的数据预留空间不够了，就会重新分配一片足够大的内存，
/// 把之前的值拷贝过来，然后释放掉旧的内存，这样就会让 data 1 中保存的 &data[0] 失效，
/// 导致内存安全问题

fn main() {
    let mut data = vec![1, 2, 3];

    let data1 = vec![&data[0]];

    println!("data[0]: {:p}", &data[0]);

    for i in 0..100 {
        data.push(i);
    }
    println!("data[0]: {:p}", &data[0]);

    println!("{:?}", &data1);
}