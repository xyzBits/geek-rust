use async_prost::AsyncProstStream;
use futures::{SinkExt, StreamExt};
use futures::future::ok;
use tokio::net::TcpListener;
use tracing::info;
use kv::{CommandRequest, CommandResponse};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // tracing_subscriber::fmt::init();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .init();

    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;

    info!("Start listening on {}", addr);
    // println!("Start listening on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connected", addr);
        // println!("Client {:?} connected", addr);

        tokio::spawn(async move {
            let mut stream =
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>
                ::from(stream)
                    .for_async();

            while let Some(Ok(msg)) = stream.next().await {
                info!("Got a new command: {:?}", msg);
                // println!("Got a new command: {:?}", msg);

                // 创建一个 404 返回给客户端
                let mut resp = CommandResponse::default();
                resp.status = 404;
                resp.message = "Not Found".to_string();

                stream.send(resp).await.unwrap();

                info!("Client {:?} disconnected", addr);
                // println!("Client {:?} disconnected", addr);
            }
        });
    }


}