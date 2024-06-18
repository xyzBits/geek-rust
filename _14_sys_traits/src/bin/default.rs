use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

// Clone trait 可以通过派生宏实现，这样能简化不少代码，
// 如果在你的数据结构里，每一个字段都已经实现了 clone trait，可以用 #[derive(Clone)]
// Clone 是深度拷贝，栈内存和堆内存一起拷贝
// clone 方法的接口是 &self，这在绝大多数场合下是适用的，在clone一个数据时只需要已有数据的只读引用，
//
// struct 可以 derive default，但我们需要所有字段都实现了 default
#[derive(Clone, Debug, Default)]
struct Developer {
    name: String,
    age: u8,
    //Trait `Default` is not implemented for `Language` [E0277]
    lang: Language,
}

// enum 不能 derive default
#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

// 手工实现 Default
impl Default for Language {
    fn default() -> Self {
        Language::Rust
    }
}


impl Developer {
    pub fn new(name: &str) -> Self {
        // 用 Default::default() 为剩余字段使用缺省值
        Self {
            name: name.to_owned(),
            ..Default::default()
        }
    }
}


impl Display for Developer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "{}({} years old): {:?} developer",
               self.name, self.age, self.lang)
    }
}

fn main() {
    // 使用T::default
    let dev1 = Developer::default();

    // 使用Default::default()，此时类型无法通过上下文推断，需要提供类型
    let dev2: Developer = Default::default();

    // 使用 T::new()
    let dev3 = Developer::new("bitch");
    println!("dev1 = {:?}", dev1);
    println!("dev2 = {:?}", dev2);
    println!("dev3 = {:?}", dev3);

    let shared = Arc::new(Mutex::new(1));
    let mut g = shared.lock().unwrap();
    *g += 1;

}