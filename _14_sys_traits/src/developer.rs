use crate::Language::Rust;

#[derive(Debug, Clone)]
struct Developer {
    name: String,
    age: u8,
    lang: Language,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

fn main() {
    let dev = Developer {
        name: "dong fang".to_string(),
        age: 12,
        lang: Language::Rust,
    };

    let cloned_dev = dev.clone();

    println!("dev: {:?}, addr of dev name: {:p}", dev, dev.name.as_str());
    println!("cloned_dev: {:?}, addr of cloned_dev name: {:p}", cloned_dev, cloned_dev.name.as_str());
}