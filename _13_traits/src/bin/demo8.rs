/// 静态分派固然很好，效率很高，但是很多时候，类型可能很难在编译时决定，比如要撰写一个格式化工具，
/// 这个在IDE 中很常见，我们可以定义一个 Formatter 接口，然后创建一系列实现


pub trait Formatter {
    fn format(&self, input: &mut String) -> bool;
}


struct MarkdownFormatter;

impl Formatter for MarkdownFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("\nformatted with Markdown formatter");
        true
    }
}


struct RustFormatter;

impl Formatter for RustFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("\nformatted with Rust formatter");
        true
    }
}

struct HtmlFormatter;

impl Formatter for HtmlFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("\nformatted with HTML formatter");
        true
    }
}


/// 首先，使用使用格式化方法，只有当打开文件时，分析出文件内容之后才能确定，
/// 我们无法在编译期给定一个具体类型，其次，一个文件可能有一到多个格式化工具，
/// 比如一个 Markdown 文件里有 Rust 代码，同时需要 MarkdownFormatter 和
/// RustFormatter 来格式化
///
/// 这里如果使用一个 Vec<T> 来提供所有需要的格式化工具，那么，下面这个函数其
/// formatters 参数该如何确定类型呢
///
/// 正常情况下，Vec<> 容器里的类型需要是一致的，但此处无法给定一个一致的类型
///
///
// pub fn format(input: &mut String, formatters: Vec<impl Formatter>) {
//     for formatter in formatters {
//         formatter.format(input);
//     }
// }

/// 所以我们要有一种手段，告诉编译器，此处需要并且仅需要任何实现了 Formatter 接口
/// 的数据类型，在 Rust 里，这种类型叫 Trait Object，表现为 &dyn Trait
/// 或者 Box<dyn Trait>
///
/// 这里，dyn 关键字只是用来帮助我们更好的区分普通类型和 Trait 类型，阅读代码时，
/// 看到 dyn 就知道后面跟的是一个 trait 了
///
/// 这样可以在运行时，构造一个 Formatter 列表，传递给 format 函数进行文件的格式化，这就是动态分派
///
// pub fn format(input: &mut String, formatters: Vec<Box<dyn Formatter>>) {
pub fn format(input: &mut String, formatters: Vec<&dyn Formatter>) {
    for formatter in formatters {
        formatter.format(input);
    }
}

/// Trait object 是 Rust 独有的概念，但是这个概念并不新鲜，
///
/// Trait Object 的实现机理
///
/// 当需要使用 Formatter trait 做动态分派时，
/// 想象如下的例子，将一个具体类型的引用，赋给 &Formatter
///
/// HtmlFormatter 的引用赋给 Formatter 之后，会生成一个 Trait Object，
/// 在上图中可以看到，Trait object 的底层逻辑就是胖指针，
/// 一个指向数据本身，另一个则指向虚函数表
///
/// vtable 是一张静态的表，Rust 在编译时会为使用了 trait Object 的类型的 trait 实现
/// 生成一张表，放在可执行文件中，
///
/// 在这张表里，包含具体类型的一些信息，size aligment 以及一系列函数 指针
/// 这个接口支持的所有方法，比如 format()
/// 具体类型的 drop trait，当trait object 被释放时，它用来释放其使用的所有资源
///
/// 这样，当在运行时执行 formatter.format() 时，formatter 就可以从 vtable 里找到
/// 对应的虚函数指针，执行具体的操作


// Clone cannot be make into an object
// 如果 trait 的所有方法，返回值都是 Self，或者携带泛型的参数，那么这个 trait 就不能产生 trait object
fn clone_test(input: Vec<Box<dyn Clone>>) {

}


fn main() {

    let mut text = "hello world".to_string();

    let html: &dyn Formatter = &HtmlFormatter;
    let rust: &dyn Formatter = &RustFormatter;

    let formatters = vec![html, rust];

    format(&mut text, formatters);

}