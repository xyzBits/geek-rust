use std::future::Future;
use std::pin::Pin;

use tokio::fs;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filename = "/tmp/async_internals";
    write_hello_file_async(filename).await?;

    Ok(())
}

async fn write_hello_file_async(name: &str) -> anyhow::Result<()> {
    let mut file = fs::File::create(name).await?;

    file.write_all(b"hello world").await?;

    Ok(())
}

enum WriteHelloFile {
    Init(String),
    AwaitingCreate(Pin<Box<dyn Future<Output=Result<fs::File, std::io::Error>>>>),
    AwaitingWrite(Pin<Box<dyn Future<Output=Result<(), std::io::Error>>>>),
}

#[allow(dead_code)]
impl WriteHelloFile {
    pub fn new(name: impl Into<String>) -> Self {
        Self::Init(name.into())
    }
}

#[allow(dead_code)]
fn write_hello_file_async1(name: &str) -> WriteHelloFile {
    WriteHelloFile::new(name)
}

