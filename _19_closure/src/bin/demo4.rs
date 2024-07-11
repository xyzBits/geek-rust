

/// 如果一个闭包不转移自己的内部数据，那么它就不是 FnOnce，然而，一旦它被当做 FnOnce 调用，
/// 自己会被转移 到 call_once 函数的作用域中，
/// 之后就无法再次调用了，
///

fn main() {
    let name = "tom".to_owned();

    // 这个闭包会 clone 内部的数据返回，所以它不是 FnOnce
    let c = move |greeting: String| (greeting, name.clone());

    // 所以 c 可以被调用多次
    println!("c call once: {:?}", c("hello".into()));
    println!("c call twice: {:?}", c("hi".into()));

    // 然而一旦它被 当成 FnOnce 调用，就无法被再次调用
    println!("result: {:?}", call_once("hi".into(), c));
    // 无法再次调用
    // let result = c("hi".to_string());

    // Fn 也可以被当成 FnOnce 调用，只要接口一致就可以
    println!("result: {:?}", call_once("hola".into(), not_closure));

}


fn call_once(args: String, c: impl FnOnce(String) -> (String, String)) -> (String, String) {
    c(args)
}

fn not_closure(args: String) -> (String, String) {
    (args, "Rosie".into())
}