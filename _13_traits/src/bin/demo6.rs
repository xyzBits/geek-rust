use std::ops::Add;

/// 上面的例子，都使用了默认的泛型参数，那定义泛型有什么用
///
/// 用加法的例子，来回答这个问题，之前都是两个复数相加，现在设计一个复数 和 一个实数相加
/// 相加的结果是 实部和实数相加，虚部不变，
#[derive(Debug)]
struct Complex {
    real: f64,
    imagine: f64,
}

impl Complex {
    pub fn new(real: f64, imagine: f64) -> Self {
        Self {
            real,
            imagine,
        }
    }
}

/// 通过使用 Add，为 Complex 实现了和 f64 相加的方法，所以泛型可以让我们
/// 在需要的时候，对同一种类型的同一个 trait ，有多种实现
impl Add<f64> for &Complex {
    type Output = Complex;

    fn add(self, rhs: f64) -> Self::Output {
        let real = self.real + rhs;

        Complex::new(real, self.imagine)
    }
}
fn main() {
    let c1 = Complex::new(2.3, 8.1);
    let result = &c1 + 5.9f64;
    println!("{:?}", result);
}