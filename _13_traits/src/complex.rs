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

impl std::ops::Add for Complex {
    type Output = Self;
    // add 的第一个参数是self，会移动所有权
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imagine: self.imagine + rhs.imagine,
        }
    }
}


// 如果不想移动所有权，可以为 &Complex 实现add，这样可以做 &c1 + &c2
impl std::ops::Add for &Complex {
    // 返回值不能是Self了，因此此时 Self 是&Complex
    type Output = Complex;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imagine: self.imagine + rhs.imagine,
        }
    }
}

// 因为 Add<Rhs = Self> 是个泛型 trait ，我们可以为 Complex 实现 Add<f64>
impl std::ops::Add<f64> for &Complex {
    type Output = Complex;
    fn add(self, rhs: f64) -> Self::Output {
        Complex::new(self.real + rhs, self.imagine)
    }
}

fn main() {
    let c1 = Complex::new(1.0, 1f64);
    let c2 = Complex::new(2f64, 3.0);

    let complex = &c1 + &c2;
    println!("{:?}", complex);

    let complex1 = &c1 + 5.9f64;
    println!("{:?}", complex1);

    let complex2 = c1 + c2;
    println!("{:?}", complex2);
    // c1 c2 已经被移动，下面再打印，将无法编译
}