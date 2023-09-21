use std::collections::HashMap;
use std::sync::Arc;

fn main() {
    let mut map = HashMap::new();
    map.insert("hello", "world");

    let key = "hello1";

    // 按照之前的说法，这段代码无法通过编译，因为同一个作用域下不能有两个可变引用
    // 当none时，map.get_mut() 的引用实际已经结束
    // match map.get(key) {// 可变引用的生命周期一直持续到 match 结果
    match map.get_mut(key) {// 可变引用的生命周期一直持续到 match 结果
        Some(v) => do_something(v),
        None => {
            map.insert(key, "dong fang");// 这里又获得了一个可变引用
        }
    }

    println!("{:?}", map);
    let s = Arc::new("hello world".to_string());
    println!("{}", s);
}

fn do_something(_v: &mut &str) {
    todo!()
}