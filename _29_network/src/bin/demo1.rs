use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

/// Rust 标准库提供了 std::net 为整个 TCP/IP 协议进行了封装，然后std::net 是同步的
/// tokio::net 提供了几乎和 std::net 一样的封装，但它是异步的
///
/// TCP: TcpListener / TcpStream 处理服务器的监听以及客户端的连接
/// UDP: UdpSocket  处理 UDP Socket
/// IpAddr 是 IPv4 IPv6 的封装，SocketAddr 表示  Ip + 端口的数据结构
///
/// TcpListener / TcpStream
/// 如果要创建一个 TCP Server ，可以使用 TcpListener 绑定某个端口，
/// 然后用 loop 循环处理收到的客户端请求，收到请求后，会得到一个 TcpStream，
/// 它实现了 Read Write trait，可以像读写文件一样，对 Socket 进行读写
///
/// 目前客户端和服务器都需要硬编码接收数据的大小，这样不够灵活，后面会看到使用消息帧 frame 来更好的处理
///
/// 处理网络连接的一般方法，
///
/// 如果使用某个 Web Framework 处理 Web 流程，那么无需关心网络流量，
/// 框架会帮你打点好一切，你只需要关心某个路由或者某个 RPC 的处理逻辑就可以了
/// 但如果你需要在 TCP 上构建自己的协议，就需要认真考虑如何妥善处理网络连接
///
/// 在listener 中可以看到，在网络处理的主循环中，会不断 accept 一个新的连接
/// 但是，处理连接的过程中，需要放在另一个线程或者另一个异步任务中进行，
/// 而不要在主循环中处理，
/// 因为这样会阻塞主循环，使其在处理完当前连接前，无法 accept 新的连接
///
/// 但是，使用线程处理频繁连接和退出的网络连接，一为会有效率上的问题，二来线程间如何共享公共的数据也让人头疼
///

fn main() {
    let listener = TcpListener::bind("0.0.0.0:9527").unwrap();
    loop {
        let (mut stream, addr) = listener.accept().unwrap();
        println!("Accepted a new connections: {}", addr);

        thread::spawn(move || {
            let mut buf = [0u8; 12];
            stream.read_exact(&mut buf).unwrap();
            println!("data: {:?}", String::from_utf8_lossy(&buf));

            // 一共写了17个字节
            stream.write(b"glad to meet you!").unwrap();
        });
    }
}