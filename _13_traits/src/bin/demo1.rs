use std::fmt::{Debug, Formatter};
use std::io::Write;

/// 在 trait 中，方法可以有缺省的实现
/// 如果将 trait 比作父类，实现 trait 的类型比作子类，
/// 那么缺省实现的方法就相当于子类中可以重载但不是必须重载的方法
///
/// Self 代表当前类型，比如 File 类型实现了 Write，那么实现过程中使用到的 Self 就指代 File
/// self 在用方法的第一个参数时，实际上是 self: Self 的简写，所以 &self 是 self: &Self
/// 而 &mut self 是 self: &mut Self
///


struct BufBuilder {
    buf: Vec<u8>,
}

impl BufBuilder {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(1024),
        }
    }
}


// 实现 Debug trait 打印字符串
impl Debug for BufBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.buf))
    }
}

impl Write for BufBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // 把 buf 添加到 BufBuilder 的尾部
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // 由于是在内存中操作，所以不需要 flush
        Ok(())
    }
}


fn main() {}