
///

#[allow(dead_code)]
enum Language {
    Rust,
    TypeScript,
    Java,
    Python,
}


impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        match self {
            Language::Rust => {"Rust"}
            Language::TypeScript => {"TypeScript"}
            Language::Java => {"Java"}
            Language::Python => {"Python"}
        }
    }
}


fn print_ref(value: impl    AsRef<str>) {
    println!("value = {}", value.as_ref());
}

fn main() {
    let lang = Language::Rust;

    // &str 实现了 AsRef<str>
    print_ref("Hello world");

    // String 实现了 AsRef<str>
    print_ref("Hello world".to_string());

    // 我们自定义的类型 enum 也实现了 AsRef<str>
    print_ref(lang);










}