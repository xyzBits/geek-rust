use std::ops::Add;

/// trait 的定义是不是也可以支持泛型呢
///
/// 比如要定义一个 Concat trait 允许数据结构拼接起来，那么自然而然地，
/// 我们希望 String 和 String 拼接，和  &str 拼接，甚至任何能够转换成 String 的
/// 数据结构 拼接，这个时候，trait 就需要支持泛型了
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

/// 注意 add 的第一个参数是 self，它会移动所有权，所以调用完两个复数 c1 + c2 后，
/// 根据所有权规则，他们就无法使用了
///
/// 执行一次加法，原有的值就无法使用，很不方便，怎么办，能不能对 Complex 的引用实现 Add trait 呢
impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;

        Self::new(real, imagine)
    }
}

/// 如果不想移动所有权，可以为 &Complex 实现 add，这样可以做 &c1 + &c2
impl Add for &Complex {
    // 注意返回值不应该是 Self，因此 此时 Self 是 &Complex

    type Output = Complex;
    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;

        Complex::new(real, imagine)
    }
}

fn main() {

    let c1 = Complex::new(1.0, 1f64);
    let c2 = Complex::new(2f64, 3.4);

    // 使用引用进行相加，这样就不会移动所有权 
    println!("{:?}", &c1 + &c2);

    println!("{:?}", c1 + c2);

}