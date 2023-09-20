use std::collections::BTreeMap;
use std::net::SocketAddr;

fn main() {
    let mut map = BTreeMap::new();
    // 如果下面这一行去掉，不insert，编译报错 cannot infer type parameter K
    // rust 需要足够的上下文进行类型推导
    map.insert("hello", "world");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];


    // collect 是 iterator trait 的方法，将一个iterator 转为一个集合，因为很多集合类型，vec hashmap
    // 等都实现了iterator，所以这里collect 要返回什么类型，编译器是无法从上下文中推断的
    // let even_number = numbers.into_iter()

    // 必须给even_numbers 一个明确的类型，这里只是无法推断出集合类型，但集合内部元素的类型，还是可以根据上下文得出
    // 所以简写成 Vec<_>
    let even_numbers: Vec<_> = numbers.clone().into_iter()
        .filter(|n| n % 2 == 0)
        .collect();

    // 除了给变量一个显式的类型外，我们也可以让collect 返回一个明确的类型
    let odd_numbers = numbers.clone().into_iter()
        .filter(|n| n % 2 != 0)
        .collect::<Vec<_>>();// 在泛型函数后，使用::<T>来强制使用类型T

    let list = vec![332, 34, 2, 998, 1023, 4322];
    // 泛型函数调用时，在函数名后强制指定类型 ::<T> ，这里其实可以根据list的类型来判断
    let largest = largest::<i32>(list.as_ref());
    println!("largest = {}", largest);




    println!("============================转换 ip 地址和端口 =======================");
    // parse::<T>() 在这里指定了函数的泛型参数
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    println!("addr: {:#?}, port: {:#?}", addr.ip(), addr.port());

    println!("43.5 parse = {}", "43.5".parse::<f32>().unwrap());


    println!("Hello, world!");
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for i in 1..list.len() {
        let item = list[i];
        if item > largest {
            largest = item
        }
    }
    largest
}
