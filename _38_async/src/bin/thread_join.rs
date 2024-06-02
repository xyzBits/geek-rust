use std::thread;
use std::time::Duration;

fn main() {
    let first = thread::spawn(|| {
        for i in 1..100 {
            thread::sleep(Duration::from_millis(110));
            println!("first thread, name: {:?}, index: {}", thread::current().name(), i);
        }
        51
    });

    let second = thread::spawn(|| {
        for i in 1..100 {
            thread::sleep(Duration::from_millis(100));
            println!("second thread, name: {:?}, index: {}", thread::current().name(), i);
        }
        52
    });

    let r1 = first.join().unwrap();
    println!("r1 = {}", r1);
    let r2 = second.join().unwrap();
    println!("r2 = {}", r2);


}