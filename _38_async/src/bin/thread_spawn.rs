use std::thread;
use std::time::Duration;

fn main() {

    thread::spawn( || {
        for i in 0..100 {
            println!("thread name: {:?}, index: {}", thread::current().name(), i);
            thread::sleep(Duration::from_millis(300));
        }
    });

    println!("have submit task");
    // 如果这里没有其他等待操作，程序就直接结束了
    thread::sleep(Duration::from_millis(10_000));

    println!("task finished");




}