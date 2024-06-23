use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(vec![0]));

    let arc = data.clone();
    thread::spawn(move || {
        arc.lock().unwrap().push(3);
    }).join().unwrap();


    let inner = Arc::try_unwrap(data).unwrap().into_inner();

    println!("{:?}", inner);
}