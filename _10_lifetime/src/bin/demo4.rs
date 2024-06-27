fn main() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();

    let hello = strtok(&mut s1, ' ');

    println!("hello = {}", hello);
    println!("s1 = {}", s1);
    println!("s = {}", s);


}
/// 根据所有权规则，值的生命周期可以确定，它可以一直存活到所有者离开作用域，
/// 而引用的生命周期不能超过值的生命周期，
/// 在同一个作用域下，这是显而易见的，
/// 然而，当发生函数调用时，编译器需要通过函数的签名来确定，
/// 参数和返回值之间生命周期的约束
///
/// &str 不是静态区域内存的指针，&str 是一个字符串切片
/// 一个带有长度的胖指针，指向字符串的实际位置，
/// 它可以指向 hello world ，此时指向了 rodata string section 中的 hello world 的地址
/// 它的生命周期是 'static
/// 也可以指向 "hello world".to_string()
/// 此时指针指向了这个字符串的地址，生命周期是 'a
///
/// strtok 并不想改变原有的字符串，只是改变指向这个字符串的可变引用
/// 
pub fn strtok<'b, 'a>(s: &'b mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}