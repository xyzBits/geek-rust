// 在rust中，函数是一等公民，可以作为参数或者返回值

// 函数作为参数
fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}

/// rust函数参数的类型和返回值的类型都必须显式定义，如果没有返回值可以省略，返回unit,
/// 函数内部如果提前返回，需要用 return 关键字，否则 最后一个表达式就是其返回值，
/// 如果最后一个表达式加了分号，隐含其返回值为nuit

fn pi() -> f64 {
    std::f64::consts::PI
}

fn not_pi() -> () {}


/// 定义变量时，可以添加 mut 关键字让变量具备可变性
/// 默认变量不可变，是一个很重要的特性，它符合最小权限原则
/// 当你使用 mut 却没有修改变量时，会提示移除
///
/// 函数是一等公民，可以作为函数或者返回值
///
/// 函数内部如果提前返回，需要用 return 关键字，否则最后一个表达式就是其返回值
/// 如果最后一个表达式都加了分号 ; 隐含其返回值为 unit
fn main() {
    println!("apply square: {}", apply(2, square));
    println!("apply cube: {}", apply(2, cube));

    let is_pi = pi();
    let is_unit = not_pi();
    let is_unit2 = {
        pi();
    };

    println!("is_pi: {:?}, is_unit: {:?}, is_unit2: {:?}", is_pi, is_unit, is_unit2);
}
