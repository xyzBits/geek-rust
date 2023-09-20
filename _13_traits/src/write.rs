use std::fmt;
use std::io::Write;

struct BufBuilder {
    buf: Vec<u8>,
}

impl BufBuilder {
    fn new() -> Self {
        Self {
            // 包含1024个字节的一个字节数组 ，是一个动态大小的字节数组
            buf: Vec::with_capacity(1024),
        }
    }
}

impl fmt::Debug for BufBuilder {
    /// Formatter 用于格式化输出的上下文对象
    /// Result 代表了格式化操作的结果
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // String::from_utf8_lossy 是用于将字节数组转换成字符串的方法之一，它的作用是将一个 &[u8]类型的字节数组转换为一个String类型的字符串
        // lossy 是有损的，因为他能够处理包含无效utf-8编码的字节数组，而不会导致程序panic ,
        // 会将无效的utf-8 字节序列 替换为 U+FFFD 表示无效字符
        // 适用于处理来自外部数据源的字节数组
        write!(f, "{}", String::from_utf8_lossy(self.buf.as_ref()))
        // as_ref 是 将Vec转换为一个引用的方法，返回一个 &[T] 类型的引用，
        // 如果你只想传递一个不可变引用，而不是所有权，就可以调用这个函数
    }
}


impl Write for BufBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}



fn main() {
    let mut buf = BufBuilder::new();
    buf.write_all(b"hello world").unwrap();
    println!("{:?}", buf);
}
