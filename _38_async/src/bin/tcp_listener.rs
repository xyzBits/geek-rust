use anyhow::Result;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LinesCodec};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("listen to: {addr}");

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("accepted: {:?}", addr);

        tokio::spawn(async move {
            let framed = Framed::new(stream, LinesCodec::new());

            let (mut writer, mut reader) = framed.split();

            while let Some(Ok(line)) = reader.next().await {
                writer.send(format!("I got: {}", line)).await?;
            }
            Ok::<_, anyhow::Error>(())
        });
    }
}