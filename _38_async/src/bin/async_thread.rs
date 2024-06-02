use std::future::Future;
use std::thread;
use std::thread::JoinHandle;

use futures::executor::block_on;

fn main() {
    let t1 = thread_async();

    let t2 = task_async();

    // 线程一旦启动，就开始执行，这里的join不过是等待结果
    let r1 = t1.join().unwrap();
    let r2 = block_on(async move { t2.await });

    assert_eq!(r1, r2);
}

fn thread_async() -> JoinHandle<usize> {
    thread::spawn(move || {
        println!("hello world");
        42
    })
}

#[allow(clippy::all)]
fn task_async() -> impl Future<Output=usize> {
    async {
        println!("hello async");
        43
    }
}