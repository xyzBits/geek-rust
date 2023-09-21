
const PI: f64 = std::f64::consts::PI;
static E: f32 = std::f32::consts::E;
// static const 用于定义全局变量，他们在不同的上下文中使用，所以必须指定类型
fn main() {
    const V: u32 = 10;
    static V1: &str = "hello";
    println!("{}", format!("{PI}, {E}, {V}, {V1}"));
}