use std::cell::RefCell;
use std::rc::Rc;


/// 多个线程访问同一块内存，是否可以使用Rc 来处理呢
///
/// 不行，因为Rc为了性能，使用的不是线程安全的引用计数器，
/// 因此，需要使用另一个引用计数的智能指针，
/// Arc
/// 它实现了线程安全的引用计数
///
/// Arc 内部的引用计数使用了 Atomic usize，而非普通的 usize，
/// 从名称上也可以感觉出，Atomic usize 是 usize 的原子类型，
/// 它使用了 CPU 的特殊指令，来保证多线程下的安全，
///
/// Rust 实现两套不同的引用计数数据结构，完全是为了性能考虑，
///
/// 如果不用跨线程访问，可以用效率非常高的 Rc，如果要跨线程，必须要用 Arc
///
/// RefCell 也不是线程安全的，在多线程中，使用内部可变性，提供了 Mutex RwLock
///
///
///
/// Mutex 是互斥量，获得互斥量的线程对数据独占访问，
/// RwLock 是读写锁，获得写锁的线程对数据独占访问，
/// 但当没有写锁的时候，允许有多个读锁，读写锁的规则和 Rust 的借用规则 非常类似
///
///
///
/// 如果想绕过 一个值只有一个所有者 的限制 ，可以使用 Arc Rc 这样的带引用计数的智能指针
///
///
#[derive(Debug)]
struct Node {
    id: usize,
    // 使用 Rc<RefCell<Node>> 让节点可以被修改
    downstream: Option<Rc<RefCell<Node>>>,
}


impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downstream: Rc<RefCell<Node>>) {
        self.downstream = Some(downstream);
    }

    pub fn get_downstream(&self) -> Option<Rc<RefCell<Node>>> {
        self.downstream.as_ref().map(|v| v.clone())
    }
}

fn main() {


    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);

    let node4 = Node::new(4);

    node3.update_downstream(Rc::new(RefCell::new(node4)));
    node1.update_downstream(Rc::new(RefCell::new(node3)));

    node2.update_downstream(node1.get_downstream().unwrap());

    println!("node1: {:?}", node1);
    println!("node2: {:?}", node2);

    let node5 = Node::new(5);
    let node3 = node1.get_downstream().unwrap();

    node3.borrow_mut().downstream = Some(Rc::new(RefCell::new(node5)));

    println!("node1: {:?}", node1);
    println!("node2: {:?}", node2);
}