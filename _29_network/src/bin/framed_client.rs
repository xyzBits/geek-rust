use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

/// 和刚才的 TcpListener TcpStream 相比，双方都不需要知道对方发送的数据长度，
/// 就可以通过 streamExt trait 的 next() 接口获得下一个消息，
/// 在发送时，只要调用 SinkExt trait 的 send() 接口发送，
/// 相应的长度就会被自动计算并添加到要发送的消息帧的开头
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:9527").await?;

    let mut stream = Framed::new(stream, LengthDelimitedCodec::new());

    // 这里发送或者接收到的二进制 ，就像是 protobuf 序列化后的一样
    stream.send(Bytes::from("hello world")).await?;


    // 接收从服务器返回的数据
    if let Some(Ok(data)) = stream.next().await {
        println!("Got: {:?}", String::from_utf8_lossy(&data));
    }

    Ok(())
}