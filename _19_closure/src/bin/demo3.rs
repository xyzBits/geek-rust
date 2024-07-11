
/// Rust 的闭包类型
///
/// FnOnce FnMut Fn 三种闭包类型有什么区别
///
/// 在声明闭包的时候，我们并不需要指定闭包需要满足的约束，
/// 但是当闭包作为函数的参数或者数据结构的一个域时，我们需要告诉调用者，
/// 对闭包的约束，
///
/// FnOnce 中的 call_once 第一个参数是 self ，会转移 self 所有权到 call_once 函数 中
/// 这是为什么 FnOnce 被称作Once：它只能被调用一次，再次调用，
/// 编译器就会报变量已经被 move 这样的所有权错误
///
/// 这个闭包 c 啥也没做，只是把捕获的参数返回，就像一个结构体里，某
/// 个字段被转移走之后，就不能再访问，
/// 闭包内部的数据一旦被转移 ，这个闭包就不完整了，
/// 也就无法再次使用，所以宛是一个FnOnce 的闭我
fn main() {

    let name = String::from("tom");

    // 这个闭包什么也不干，只是把捕获的参数返回去
    let c = move |greeting: String|(greeting, name);

    let result = c("hello".to_string());

    println!("result = {:?}", result);

    // 无法再次调用
    // let result = c("hi".to_string());
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Self {
            name,
            age,
        }
    }

    fn set_name(mut self, name: String) {
        self.name = name;
    }
}

#[test]
fn test_self() {
    let person = Person::new("hello".to_owned(), 33);

    // person.set_name("world".to_string());

    // value borrowed here after move
    // println!("{:?}", person);
    // 在这里，name 的所有权已经被转移出去
    let data = person.name;

    // 在这里，再次打印，就会出错
    // println!("{}", person.name);

}