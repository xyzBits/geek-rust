use std::{fs, thread};
use std::thread::JoinHandle;
use std::time::Duration;

use anyhow::{anyhow, Result};
use toml::Value;

struct MyJoinHandle<T>(JoinHandle<Result<T>>);

impl<T> MyJoinHandle<T> {
    pub fn thread_await(self) -> Result<T> {
        self.0.join().map_err(|_| anyhow!("failed"))?
    }
}

fn main() -> Result<()>{
    let handle = thread::spawn(|| {
        println!("hello world");
    });

    // waits for associated thread finished
    handle.join().unwrap();


    let t1 = thread_read("./Cargo.toml");
    let t2 = thread_read("./Cargo.lock");


    let content1 = t1.thread_await()?;
    let content2 = t2.thread_await()?;


    // 计算
    let yaml1 = toml2yaml(&content1)?;
    let yaml2 = toml2yaml(&content2)?;

    // 写入 /tmp/Cargo.yml，IO 操作 3
    let t3 = thread_write("/tmp/Cargo.yml", yaml1);
    // 写入 /tmp/Cargo.lock，IO 操作 4
    let t4 = thread_write("/tmp/Cargo.lock", yaml2);

    let yaml1 = t3.thread_await()?;
    let yaml2 = t4.thread_await()?;

    fs::write("/tmp/Cargo.yml", &yaml1)?;
    fs::write("/tmp/Cargo.lock", &yaml2)?;

    // 打印
    println!("{}", yaml1);
    println!("{}", yaml2);

    Ok(())
}

fn thread_read(filename: &'static str) -> MyJoinHandle<String> {
    let handle = thread::spawn(move || {
        let s = fs::read_to_string(filename)?;
        Ok::<_, anyhow::Error>(s)
    });
    MyJoinHandle(handle)
}

fn thread_write(filename: &'static str, content: String) -> MyJoinHandle<String> {
    let handle = thread::spawn(move || {
        fs::write(filename, &content)?;
        Ok::<_, anyhow::Error>(content)
    });

    MyJoinHandle(handle)
}

fn toml2yaml(content: &str) -> Result<String> {
    let value = toml::from_str::<Value>(content)?;
    Ok(serde_yaml::to_string(&value)?)
}


