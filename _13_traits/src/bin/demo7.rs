
/// trait 的继承
/// 在 Rust 中，一个 trait 可以 继承 另一个 trait 的关联类型和关联函数，
/// 比如 trait B: A，可以说任何类型，如果实现了 trait B，它也必须实现 trait A
/// 换句话说，trait B 在定义时，可以使用 trait A 中的关联类型和关联方法
///
/// 可继承对扩展 trait 的能力很有帮助 ，很多常见的 trait 都会使用 trait 继承来提供更多的能力
///
/// trait 作为对不同数据结构 中相同行为的一种抽象 ，
/// 当行为和具体的数据关联时，比如字符串解析时定义 parse trait，我们引入 了带有关联类型的 trait，
/// 把和行为有关的数据类型的定义，进一步延迟到 trait 实现的时候
///
/// 对于同一个类型的 同一个 trait 行为，可以有不同的实现，比如我们之前大量使用的 From，此时可以用 泛型 trait
///
/// 而特设多态就是同一种行为的不同实现，所以其实，通过定义 trait 以及为不同的类型实现 这个 trait，
/// 我们就已经实现了特设多态，
///
/// Add trait 就是一个典型的特设多态，同样是加法操作，根据操作数据的不同
/// 进行不同的处理，service trait 是一个不那么明显的特设多态，同样是web请求，
/// 对于不同的 url，我们使用不同的代码去处理
///
/// 如何做子类型多态、
/// 从严格意义上讲，子类型多态是面向对象语言的专利，如果一个对象A是对象B的子类，
/// 那么A的实例可以出现 在任何期望B的实例的上下文中，比如 猫和狗都是动物，如果
/// 一个函数的接口要求传入一个动物，那么传入猫和狗都是允许 的
///
/// Rust 虽然没有父类和子类，但 trait 和实现 trait 的类型之间也是类似的关系，
/// 所以 rust 可以做子类型多态，


struct Cat;
struct Dog;


trait Animal {
    fn name(&self) -> &'static str;
}


impl Animal for Cat {
    fn name(&self) -> &'static str {
        "Cat"
    }
}

impl Animal for Dog  {
    fn name(&self) -> &'static str {
        "Dog"
    }
}

fn name(animal: impl Animal) -> &'static str {
    animal.name()
}

/// impl Animal 是 T: Animal 的缩写，所以 name 函数的定义和以下定义等价
/// 上一次提到过，这种泛型函数会根据具体使用的类型被单态化，编译成多个实现，是静态分派
///

fn name_v2<T: Animal>(animal: T) -> &'static str {
    animal.name()
}

fn main() {

    let cat = Cat;
    println!("cat: {}", name(cat));

}