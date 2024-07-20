use std::io::{Read, Write};
use std::net::TcpStream;

/// 对于客户端，可以使用 Tcp::connect() 得到一个 TcpStream，一旦客户端的请求被服务器接受
/// 就可以发送或者接收数据
///
/// 从客户端代码可以看出，我们无需显式的关闭 TcpStream，因为 TcpStream 内部也实现了 Drop trait
/// 使得其离开作用域时会被关闭
///
///

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:9527").unwrap();

    // 一共写了12个字节
    stream.write_all(b"hello world!").unwrap();


    // 一共收到 17 字节
    let mut buf = [0u8; 17];
    stream.read_exact(&mut buf).unwrap();
    println!("data: {:?}", String::from_utf8_lossy(&buf));
}