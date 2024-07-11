

/// FnMut
///
/// FnMut 可以被多次调用
/// 就像结构体如果想改变数据，需要用 let mut 声明一样，
/// 如果你想改变闭包捕获的数据结构，那么需要 FnMut
///

fn main() {
    let mut name = "Tom".to_owned();
    let mut name1 = "Jerry".to_owned();

    // 捕获 &mut name
    let mut c = || {
        name.push_str(" bitch");
        println!("c: {}", name);
    };

    // 捕获 mut name1 注意 name1 需要声明成 mut
    let mut c1 = move || {
        name1.push_str(" damn");
        println!("c1: {}", name1);
    };


    c();
    c1();


    call_mut(&mut c);
    call_mut(&mut c1);


    call_once(c);
    call_once(c1);

}

// 在作为参数时，FnMut 也要显式地使用 mut 或者 &mut
fn call_mut(c: &mut impl FnMut()) {
    c();
}

// 为啥 call_once 不需要 mut
fn call_once(c: impl FnOnce()) {
    c();
}