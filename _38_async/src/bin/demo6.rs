use std::thread;

use blake3::Hasher;
use futures::{SinkExt, StreamExt};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, oneshot};
use tokio_util::codec::{Framed, LinesCodec};

pub const PREFIX_ZERO: &[u8] = &[0, 0, 0];


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("listen to: {}", addr);


    // 创建 tokio task 和 thread 之间的 channel
    let (sender, mut receiver) = mpsc::unbounded_channel::<(String, oneshot::Sender<String>)>();

    // 使用 thread 处理计算密集型任务
    thread::spawn(move || {

        // 读取从 tokio task 传过来的 msg，注意这里使用的是 blocking_recv，而非 await
        while let Some((line, reply)) = receiver.blocking_recv() {
            // 计算pow
            let result = match pow(&line) {
                Some((hash, nonce)) => format!("hash: {}, nonce: {}", hash, nonce),
                None => "Not Found".to_string(),
            };

            // 把计算结果从 oneshot channel 发回
            if let Err(e) = reply.send(result) {
                println!("Failed to send: {}", e);
            }
        }
    });

    // 使用 tokio task 处理 IO 密集型任务
    loop {
        let (stream, addr) = listener.accept().await?;

        println!("Accepted: {:?}", addr);
        let sender1 = sender.clone();

        tokio::spawn(async move {
            let framed = Framed::new(stream, LinesCodec::new());
            let (mut writer, mut reader) = framed.split();

            for line in reader.next().await {
                // 为每个消息创建一个 oneshot channel ，用于发送回复
                let (replier, reply_receiver) = oneshot::channel();

                sender1.send((line?, replier))?;

                // 接收 pow 计算完成后的 hash 和 nonce
                if let Ok(v) = reply_receiver.await {
                    writer.send(format!("Pow calculated: {}", v)).await?;
                }
            }

            Ok::<_, anyhow::Error>(())
        });
    }
}

/// 使用 rayon 并发计算 u32 空间下的所有 nonce，直到找到有头 N 个 0 的哈希
pub fn pow(s: &str) -> Option<(String, u32)> {
    let hasher = blake3_bash_hash(s.as_bytes());
    let nonce = (0..u32::MAX).into_par_iter().find_any(|n| {
        let hash = blake3_hash(hasher.clone(), n).as_bytes().to_vec();
        &hash[..PREFIX_ZERO.len()] == PREFIX_ZERO
    });

    nonce.map(|n| {
        let hash = blake3_hash(hasher, &n).to_hex().to_string();
        (hash, n)
    })
}

/// 计算携带 nonce 后的哈希
fn blake3_hash(mut hasher: blake3::Hasher, nonce: &u32) -> blake3::Hash {
    hasher.update(&nonce.to_be_bytes()[..]);
    hasher.finalize()
}

/// 计算数据的哈希
fn blake3_bash_hash(data: &[u8]) -> Hasher {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher
}

