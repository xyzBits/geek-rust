use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::path::PathBuf;

/// 类型转换相关 trait
///
/// 我们经常需要在某个下文中，把一种数据结构转换成另一种数据结构
///
///
fn first() {
    let s = "".to_owned();
    // 第一种方法，为每一种转换提供一个方法
    // 把字符串转为 path
    // let v = s.to_path();

    // 把字符串转为 u64
    // let v = s.to_u64();

    // 第二种方法，为 s 要转换的类型之间实现一个 Into<T> trait
    // v 的类型可以根据上下文推导出
    // let v: PathBuf = s.into();
    // 或者也可以显式地标注 v 的类型
    // let v: u64 = s.into();
}

/// 第一种方法，要在类型 T 的实现里，要为每一种可能的转换类型提供一个方法，
/// 第二种，我们为类型 T 和 类型 U 之间的转换实现一个数据转换的 trait
/// 这样可以用同一个方法来实现不同的转换
///
/// 显然，第二种方法更好，因为它符合软件开发的开闭原则 ，
/// 软件中的对象（类，模块，函数等） 对扩展是开放的，但对修改是封闭的
/// 基于这个思路
/// Rust 提供了两套不同的 trait
/// 值类型到值用类型的转换 From<T> Into<T> TryFrom<T> TryInto<T>
/// 引用类型到引用类型的转换 AsRef<T> AsMut<T>


fn print_ip_addr(addr: impl Into<IpAddr>) {
    println!("addr: {:?}", addr.into());
}

fn main() {

    let v4: Ipv4Addr = "2.2.2.2".parse().unwrap();
    let v6: Ipv6Addr = "::1".parse().unwrap();

    print_ip_addr([1, 1, 1, 1]);
    print_ip_addr([0xef80, 0, 0, 0, 0xaede, 0x48ff, 0xfe00, 0x1122]);

    print_ip_addr(v4);
    print_ip_addr(v6);

}