use async_prost::AsyncProstStream;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tracing::info;
use tracing_subscriber::util::SubscriberInitExt;
use kv::{CommandRequest, CommandResponse};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();


    let addr = "127.0.0.1:9527";

    // 连接服务器
    let stream = TcpStream::connect(addr).await?;

    // 使用 AsyncProstStream 来处理 TCP Stream
    let mut client =
        AsyncProstStream::<_, CommandResponse, CommandRequest, _>
        ::from(stream)
            .for_async();

    // 生成一个 HSET 命令
    let cmd = CommandRequest::new_hset("table1", "hello", "world".into());

    // 发送 HSET 命令
    client.send(cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Got response: {:?}", data);
    }
    Ok(())
}