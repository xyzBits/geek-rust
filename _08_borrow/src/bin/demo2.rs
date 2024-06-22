
/// 不管有多少层的引用，最终数据都指向 data 本身，这个值的地址是固定的，
/// 但是它们引用的地址是不同的，因 为 只读引用实现了 copy trait，也就意味着
/// 引用的赋值，传参都会产生新的浅拷贝
///
/// 虽然 data 有很多的只读引用指向它，但堆上的数据依旧只有 data 一个所有者，
/// 所以值的多个引用并不会影响所有权的唯一性
///
/// 但是，马上发现新问题：一旦 data 离开作用域被释放了，如果还有引用指向 data，
/// 岂不是造成我们想极力避免的使用已经释放的内存，这样的安全问题
///
///

fn main() {
    let data = vec![1, 2, 3, 4];

    let data1 = &data;

    println!("data1 = {:p}", data1);
    println!("&1 = {:p}", &data);
    println!("&2 = {:p}", &&data);
    println!("&3 = {:p}", &&data);
    println!("&4 = {:p}", &&data);

    println!("sum of data {}", sum(data1));

    println!("data1 = {}", &data1[0]);
    println!("&1 = {}", &data[1]);
    println!("&2 = {}", &&data[2]);
    println!("&3 = {}", &&data[0]);
    println!("&4 = {}", &&data[3]);
}

fn sum(data: &Vec<i32>)  -> i32 {
    println!("address of data: {:p}, address of ref: {:p}", data, &data);
    (&&&&&data).iter().sum()
}