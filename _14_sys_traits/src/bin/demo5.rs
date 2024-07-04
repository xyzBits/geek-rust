use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

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
///
/// spawn 方法的参数是一个闭包，这个闭包需要 Send + 'static
/// 'static 意思是闭包捕获的自由变量必须是一个拥有所有权的类型，或者是一个拥有静态生命周期的引用
/// Send 意思 是，这些被捕获的自由变量的所有权可以从一个线程移动到另一个线程
///
/// 如果在线程间传递 Rc，是无法通过编译的，
/// 因为 Rc 不支持 Send 和 Sync
///

fn rc_is_not_send_and_sync() {
    let a = Rc::new(1);
    let b = a.clone();
    let c = b.clone();

    // Rc<i32> cannot be sent between threads safely
    // thread::spawn(move || {
    //     println!("c = {}", c);
    // });
}


/// RefCell 实现了 Send，但没有实现 Sync，所以，看起来是可以工作 的
fn refcell_is_send() {
    let a = RefCell::new(9);
    thread::spawn(move || {
        println!("{:?}", a);
    });

}

/// Rc 不能Send，我们无法跨线程使用 Rc<RefCell<T>> 这样的数据结构 ，
///那么使用 Send Sync 的 Arc 呢，
///使用 Arc<RefCell<T>> 来获得，一个可以在多线程之间共享，
///且可以修改的类型，可以么
///
/// 不可以，因为 Arc 内部的数据是共享的，需要支持 Sync 的数据结构，
/// 但 RefCell 不是 Sync，编译失败，所以在多线程的情况下，
/// 我们只能使用 支持 Sync Send 的 Arc
/// 和 Mutex 一起，构造一个可以在多线程间共享且可以修改的类型
fn refcell_is_not_sync() {
    let a = Arc::new(RefCell::new(1));
    let b = a.clone();
    let c = a.clone();

    // thread::spawn(move || {
    //     println!("c = {:?}", c);
    // });
}


fn arc_mutex_is_send_sync() {
    let a = Arc::new(Mutex::new(1));
    let b = a.clone();
    let c = a.clone();

    let handle = thread::spawn(move || {
        let mut g = c.lock().unwrap();
        *g += 2;
    });

    {
        let mut g = b.lock().unwrap();
        *g += 3;
    }

    handle.join().unwrap();

    println!("a = {:?}", a);




}

#[allow(dead_code)]
fn main() {

    arc_mutex_is_send_sync();
}