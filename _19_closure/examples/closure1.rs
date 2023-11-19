fn main() {
    let x = 1;
    //sum 可以赋给变量，允许捕获调用者作用域中的值
    let sum = |y| x + y;
    assert_eq!(3, sum(2));
}