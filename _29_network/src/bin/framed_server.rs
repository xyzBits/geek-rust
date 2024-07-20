use bytes::Bytes;
use rocket::futures::{SinkExt, StreamExt};
use rocket::yansi::Paint;
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

/// 使用 protobuf 自定义协议
/// protobuf 生成的消息是不定长的，所以客户端和服务端之间要约定好，如何定义一个消息帧
///
/// 常用的方法是在消息的末尾添加 /r/n ，以及在消息头添加长度
/// 消息尾部添加 /r/n 一般用于基于文本的协议，比如 http3，pop3 redis resp
/// 对于二进制协议，更好的方式是在消息前面添加定义的长度
/// 比如对于 protobuf 这样的二进制而言，消息中的数据可能正好出现连续的 /r/n
/// 如果使用/r/n作 为消息的边界，就会混乱
///
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9527").await?;

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Accepted: {:?}", addr);

        // LengthDelimitedCodec 默认 4 字节 长度
        let mut stream = Framed::new(stream, LengthDelimitedCodec::new());


        tokio::spawn(async move {
            // 接收到的消息只会包含消息不体，不包含长度
            while let Some(Ok(data)) = stream.next().await {
                println!("Got: {:?}", String::from_utf8_lossy(&data));

                // 发送的消息也需要发送消息主体，不需要提供长度
                // Framed/ LengthDelimitedCodec 会自动计算并添加
                stream.send(Bytes::from("goodbye world!")).await.unwrap();
            }
        });
    }


}