
/// 示例代码中，用了很多 unwrap ，这样可以么
/// 要从 Option<T> Result<T, E> 中获得数据时，可以使用 unwrap()
/// 对于学习性质的代码，这样是可以接受的，
/// 但在生产环境中，除非你可以保证 unwrap() 不会引发 panic!()
/// 否则 应该使用模式匹配来处理数据 ，
/// 或者使用错误处理的 ? 操作符，
///
/// 什么情况下我们可以确定 unwrap 不会 panic 呢
/// 如果在做 unwrap 之前，Option<T> Result<T, E> 中已经有合适的值
/// Some(T) Ok(T)
/// 就可以 unwrap
fn main() {

}