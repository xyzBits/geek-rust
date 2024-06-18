use std::{panic, thread};
use std::thread::panicking;

fn main() {
    let origin_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        origin_hook(panic_info);
        println!("catch a panic {panic_info}");
    }));


    // to_string converts a given value to string
    let name = "bitch".to_string();
    let handle = thread::spawn(move || {
        println!("{}", name);
        panic!("bitch")
    });

    // waits for the associated thread to finish.
    // this function will return immediately if the associated thread has already finished.
    // If the associated thread panics, Err is returned with the parameter given to panic
    let result = handle.join();

    match result {
        Ok(_) => {println!("execute succeed")}
        Err(e) => println!("error is ")
    }

    // result.unwrap();

    let i = 5;
    let five = String::from("5");

    assert_eq!(five, i.to_string());

    println!("five = {five}");
}