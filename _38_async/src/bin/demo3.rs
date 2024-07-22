use std::pin::Pin;
use std::task::Context;

/// 除了 Output 外，还有一个 poll 方法，这个方法返回 Self::Output
/// 而 Poll<T> 是一个enum，包含 ready pending 两个状态，显然，
/// 当 Future 返回 Pending 状态时，活还没干完，但干不下去了，要阻塞一阵子，
/// 等某个事件的唤醒
/// 当 Future 返回 Ready 状态时， Future 对应的值已经得到，此时可以返回了
///
pub trait MyFuture {
    type Output;
    fn poll(self: Pin<&mut self>, cx: &mut Context<'_>) -> MyPoll<Self::Output>;
}

pub enum MyPoll<T> {
    Ready(T),
    Pending,
}

fn main() {

}