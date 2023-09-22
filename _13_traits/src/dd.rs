pub trait Fly {
    fn fly(&self);
}

struct Goose;

#[allow(dead_code)]
struct Duck {
    height: u8,
}

impl Goose {
    pub fn new() -> Self {
        Self
    }
}

impl Duck {
    pub fn new(height: u8) -> Self {
        Self {
            height,
        }
    }
}

impl Fly for Goose {
    fn fly(&self) {
        println!("Goose is fly");
    }
}


impl Fly for Duck {
    fn fly(&self) {
        println!("Duck is flying");
    }
}


fn fly(a: impl Fly) {
    a.fly();
}


// impl Fly 无法作为返回值，需要有某个确定的类型，这样才能编译通过
// 这段代码无法提供确定的类型，所以出错
// fn select(name: &str) -> impl Fly {
//     match name {
//         "goose" => Goose::new() as Fly,
//         "duck" => Duck::new(8) as Fly,
//     }
// }
fn main() {
    let g = Goose::new();
    let d = Duck::new(9);
    fly(g);
    fly(d);
}