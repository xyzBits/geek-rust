
/// Send / Sync
/// 这两个 trait 都是 unsafe auto trait，
/// auto 意味着编译器会在合适的场合，自动为数据结构添加它们的实现，
/// 而 unsafe 代表实现的这个 trait 可能会违背 rust 的内存安全规则 ，
/// 如果开发者手动实现 这两个 trait，要自己为他们的安全性负责
///
/// Send Sync 是并发安全的基础
///
/// 如果一个类型 T 实现了 Send trait，意味着 T 可以安全地从一个线程移动到另一个线程，
/// 也就是说所有权可以在线程间移动
///
/// 如果一个类型 T 实现了 Sync trait，意味着 &T 可以安全地在多个线程中共享，
/// 一个类型 T 满足 Sync trait，当且仅当 &T 满足 Send trait
///
/// 对于 Send Sync 在线程安全中的作用，可以这么看，如果 一个类型 T: Send，
/// 那么 T 在某个线程中的独占访问是线程安全的，
/// 如果一个类型 T: Sync 那么 T 在线程间的只读共享是安全的
///
/// 对于我们自己定义的数据结构 ，如果其内部所有域都实现了 Send Sync
/// 那么这个数据结构会被自动添加 Send Sync，基本上原生数据结构都支持 Send Sync，
/// 也就是说，绝大部分自定义数据结构是满足 Send Sync 的，标准库中，
/// 不支持 Send Sync 的数据结构主要有
///
/// 祼指针 *const / *mut T 它们是不安全的，所以既不是 Send 也不是 Sync
/// UnsafeCell<T> 不支持Sync，也就是，任何使用了 Cell 或者 RefCell 的数据结构不支持 Sync
/// 引用计数Rc 不支持 Send 也不支持 Sync ，所以 Rc无法跨线程
fn main() {

}