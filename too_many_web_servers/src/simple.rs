use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    // unwrap 是 Result Option 两个枚举类型的方法，用于从这些枚举中获取值，它是一个简便的方法，用于处理正常情况下值的获取，但是在遇到错误或者 None 的情况下会发生 panic
    // Result<T, E> 的unwrap 方法结果ok时返回T类型，如果结果是 Err(e) 则触发 panic
    // Option<T> 使用unwrap Some(T) 的情况下返回 T，如果是None则触发panic
    // 这个方法在非常确定的情况下使用，如果你认为值总是存在，但是实际情况可能不会如预期，会导致 程序崩溃
    // 了为更好的错误处理和程序稳定性，推荐使用 Result Option 的更安全该当 ，例如 match if let unwrap_or 等来处理成功和失败的情况，而不是直接 使用unwrap
    let listener = TcpListener::bind("localhost:3000").unwrap();

    loop {
        let (connection, address) = listener.accept().unwrap();
        println!("address = {}", address);
        if let Err(e) = handle_connection(connection) {// 使用模式匹配处理 error
            println!("failed to handle connection: {e}");
        }
    }
}

fn handle_connection(mut connection: TcpStream) -> io::Result<()> {
    // ....
    // 初始化数组，并给初值为0
    let mut request:[u8; 1024] = [0; 1024];
    let mut read = 0;

    loop {
        // rust 中 ？符号是用来处理 Result 或者 Option类型的错误传播的简便方式，它通常用在返回 Result Option类型的函数中，以便在发生错误时提前返回错误结果
        // 而不需要手动处理错误
        // 具体来说，当使用 ? 时，它会以当前函数返回错误值 err 的情况下，自动将错误向上传递给调用函数的地方，如果调用 的地方也用了 ?，会继续传
        // try reading from the stream

        // request 每往里面写入一些东西，就把切片往后移动
        let num_bytes = connection.read(&mut request[read..])?;

        // the client disconnected
        if num_bytes == 0 {
            println!("client disconnected unexpectedly");
            return Ok(());
        }

        // keep track of how many bytes we've read
        read += num_bytes;

        // have we reached the end of the request?
        if request.get(read - 4..read) == Some(b"\r\n\r\n"){
            break;
        }
    }

    let request = String::from_utf8_lossy(&request[..read]);
    println!("{request}");

    // "Hello World!" in HTTP
    let response = concat!(
    "HTTP/1.1 200 OK\rn\rn",
    "Content-Length: 12\r\n",
    "Connection: close\r\n\r\n",
    "Hello world!"
    );

    let mut written = 0;

    loop {
        // write the remaining response bytes
        let num_bytes = connection.write(response[written..].as_bytes())?;

        // the client disconnected
        if num_bytes == 0 {
            println!("client disconnected unexpectedly");
            return Ok(());
        }

        written += num_bytes;

        // have we written the whole response yet?
        if written == response.len() {
            break;
        }
    }
    connection.flush()
}







































