use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LinesCodec};

/// 什么是 executor
///     executor 是一个  Future 的调度器，对于线程来说，操作系统负责调度
///     但操作系统不会调度用户态的协程，比如 Future，所以任何使用了协程来处理并发的程序 ，
///     要需要有一个 executor 来负责协程的调度
///
///     很多在语言层面 支持协程的语言，都自带一个用户态的调度器，
///     Rust 虽然也提供 Future 这样的协程，但它在语言层面 并不提供 executor
///     把要不要使用 executor 以及使用什么样的 executor 的自主权交给了开发者，
///     好处是，代码中不需要使用协程时，不需要引入任何运行时，
///     而需要使用协程时，可以在生态系统中选择合适的 executor
///
/// 常见的 executor 有
///     futures
///     tokio
///     async-std
///     smol
///
///     代码中可能 混用，但只是为了演示，在正式代码中， 不建议混用，会降低程序的性能，
///     还可能引发奇怪的问题
/// executor reactor pattern 作为构建高性能事件驱动系统的一个很典型的模型，
/// Reactor pattern 它包含三部分组成
///     task 待处理的任务，任务可以被打断，并且把控制权交给 executor ，等待之后的调度
///     executor 一个调度器，维护等待运行的任务， ready queue， 以及被阻塞的任务 wait queue
///     reactor 维护事件队列，当事件来临时，通知 executor 唤醒某个任务等待运行
///
///     executor 会调度待处理的任务，当任务无法继续进行却又没有完成时，它会挂起任务，
///     并设置好合适的唤醒条件，
///     之后，如果 reactor 得到了满足条件的事件，它会唤醒之前挂起的任务，然后 executor 就有机会
///     断续执行这个任务，这样一直循环下去，直到任务执行完毕
///
/// 怎么用 Future 做异步处理
///     理解 reactor pattern 后，
///     以 tokio 为例 ，async / await 提供语法层面的支持，
///     Future 是异步任务的数据结构，
///     当 fut.await 时，executor 就会调度并执行它
///
///     tokio 的调度器 executor 会运行在多个线程上，运行线程自己的 ready queue 上的任务 future
///     如果没有，就去别个线程的调度器上偷一些过来运行，当某个任务无法再继续取得进展 ，
///     此时 Future 的运行结果是 Poll::Pending，那么调度器会挂起任务，
///     并设置好唤醒条件 Waker 等待被 reactor 唤醒
///
///     而 reactor 会利用操作系统提供的 异步 I/O ，比如 epoll / kqueue / IOCP
///     来监听操作系统的 I/O 事件，当遇到满足条件的事件时，会调度 Waker.wake() 唤醒被挂起的
///     Future，这个 Future 就会回到 ready queue 等待执行

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("listen to: {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Accepted: {}", addr);

        tokio::spawn(async move {
            let framed = Framed::new(stream, LinesCodec::new());

            let (mut writer, mut reader) = framed.split();


            // 假设客户端输入了很大的一行数据，服务器在做 reader.next().await 在执行的时候，接收不完一行数据
            // 因而这个 Future 返回 Poll::Pending，此时它被挂起，当后续客户端的数据到达时，
            // reactor 会知道这个 socket 上又有数据了，于是找到 socket 对应的 Future，将其唤醒，继续执行
            // 这样反复下去，最终  reader.next().await 得到 Poll::Ready(Ok(line))，于是它返回 Ok(line)
            // 程序继续往下去，进入到 writer.send 阶段
            for line in reader.next().await {
                writer.send(format!("I got: {}", line?)).await?;
            }
            Ok::<_, anyhow::Error>(())

        });

    }

}