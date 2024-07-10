use std::fmt::Debug;

/// 特殊的切片 &str
///

#[allow(dead_code)]
#[allow(unused)]
fn main() {
    let s = String::from("hello");

    // &String 会被解引用成 &str
    // &String -> &str
    print_slice(&s);

    // &s[..] s.as_str() 一样会得到 &str
    print_slice(&s[..]);

    // String 支持 AsRef<str>
    print_slice_1(&s);
    print_slice_1(&s[..]);
    print_slice_1(s.clone());

    // String 也实现了 AsRef<[u8]>
    //
    print_slice_2(&s);

}

fn print_slice(s: &str) {
    println!("print_slice: {}", s);
}

fn print_slice_1<T: AsRef<str>>(s: T) {
    println!("print_slice_1: {}", s.as_ref());
}

fn print_slice_2<T, U>(s: T)
where T: AsRef<[U]>,
    U: Debug {
    println!("print_slice_2: {:?}", s.as_ref());
}