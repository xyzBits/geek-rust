/// result 是一个引用，但它可能 “引自” 两个生命周期不周的变量，
/// 那么这里我们应该给 result 赋予一个什么样的生命周期
///
/// 很明显，result 应该被赋予的生命周期取决于 longest 函数的逻辑
/// 然而，一个函数的逻辑可能非常复杂，即使这里的 longest 函数的逻辑相对简单，
/// 编译器难以推导出返回值和传入参数之间的关系
/// 所以 Rust 要求函数的编写者显式地标明函数的返回值应该被赋予的生命周期
/// 与
/// 传入参数的生命周期
/// 的关系
///
/// 再 看看 longest 函数 ，它的每一个参数有自己的生命周期，
/// 比如这里的 x y 分别具有 'a 'b 的生命周期
/// 我们现在要做的就是给返回值标明一个生命周期参数
/// 例如 'c ，同时表达这个生命周期参数 和 'a 'b 的关系
///
/// Rust 的基本规则 ，
/// 要避免悬垂引用，那么必须保证，当 result 可以被访问时，
/// string1 string2 也必须能被访问
///
/// 换个说法，传入参数对应的变量 string1 string2 的drop
/// 只能发生在返回值赋予到的变量 result drop 之后，或同时
///
/// 即返回值赋予到的变量 result 的生命周期 短于 或者等于
/// 传入参数所对应变量的生命周期
///
/// 在 Rust 中，生命周期也是一种类型，生命周期长的类型可以作为
/// 生命周期短的类型的子类型
/// 'l 长于 's 记作 'l: 's
///
/// 回到上面的代码，我们可以知道，对于调用方，返回值 被 赋予的变量的生命周期 'c
/// 必须 短于 'a 'b 中短的那一个
/// shortest {'a, 'b}: 'c
///
///
fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(&string1, &string2);

    println!("The longest string is {}", result);
}

fn longest<'short_life, 'long_life : 'short_life>
(x: &'short_life str, y: &'short_life str)
 -> &'long_life str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}