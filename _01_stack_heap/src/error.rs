fn main() {
    let name = "world".to_string();// convert &str to String

    std::thread::spawn(move || {
        println!("hello, {}", name);
    });
}