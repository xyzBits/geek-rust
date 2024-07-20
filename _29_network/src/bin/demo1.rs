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