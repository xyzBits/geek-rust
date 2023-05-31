fn main() {
    let name = "dong fang".to_string();

    std::thread::spawn(move || {
        // function requires argument type to outlive `'static`
        //to force the closure to take ownership of 'name' (and any other referenced variables), use the 'move' keyword
        // may outlive borrowed value 'name'
        println!("hello {}", name);// 'name' is borrowed here
    });
}