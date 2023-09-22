trait Animal {
    fn name(&self) -> &'static str;
}

struct Cat;

struct Dog;

impl Animal for Cat {
    fn name(&self) -> &'static str {
        "cat"
    }
}

impl Animal for Dog {
    fn name(&self) -> &'static str {
        "Dog"
    }
}

fn name(animal: impl Animal) -> &'static str {
    animal.name()
}

fn main() {
    let cat = Cat;
    println!("{}", name(cat));
}