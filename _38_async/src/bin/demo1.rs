use futures::future::try_join;
use tokio::{fs, try_join};
use toml::Value;

/// javascript promise 代表了在未来某个时刻才能得到结果 的值，promise 一般存在3个状态
/// 1。 初始状态，promise 还未运行
/// 2。 等待状态，pending, promise 已运行，但还未结束
/// 3。 结束状态，promise 成功解析出一个值，或者执行失败
///
/// js 中的 promise 和线类似，一旦创建就开始执行，
/// 对 promise await 只是为了等待并获取解析出来的值，
/// 而 rust 的 future，只有主动 await 后才开始执行
///
/// async / await
///     async 定义了一个可以并发执行的任务，
///     而 await 则触发这个任务并发执行
///
///     大多数语言，包括 rust ，async/await 都是一个语法糖，
///     它们他们状态机将 promise future 这样的结构包装起来进行处理
///
///
///
/// 为什么需要 Future
///     在 Future 出现 之前，Rust 代码都是同步的，也就是说，当你执行一个函数，
///     CPU 处理完函数中的每一个指令都会返回，如果这个函数里有 IO操作，
///     实际上，操作系统会把函数对应的线程挂起，放到一个等待队列中，直到 IO 操作完成，
///     才恢复这个线程，并从挂位置继续执行下去
///
///     过于充沛的CPU 算力和提升缓慢的 IO 速度之间的矛盾
///     如果有大量的 IO 操作，你的程序大部分时间没有在运算，而是不断的等待IO
///
/// 创建过多的线程会大大增加系统的开销
///     其实，绝大多数操作系统对 I/O 操作提供了非阻塞接口，也就是说，你可以发起一个读取的指令，
///     自己处理类似 ewouldblock 这样的错误代码，来更好地在同一个线程中处理多个文件的IO
///     而不依赖操作调度帮你完成这件事
///
///     不过这样意味着，你需要定义合适的数据结构来追踪每个文件的读取，在用户态进行相应的调度，
///     阻塞等待IO的数据结构的进行，让没有等待IO的数据结构得到机会使用CPU
///     以及当IO操作结束后，恢复等待IO的数据结构的运行等，
///     这样操作的粒度更小，可以最大程度利用 CPU资源，
///
///     这样，我们需要在用户态做很多事情 ，包括处理IO任务的事件通知，
///     创建 Future，合理调度 Future，这些事情，统统交给开发者做显然 不合理
///     所以 Rust 提供了 相应处理手段 async await
///
///     async 来方便地生成 Future
///     await 来触发 Future 的执行和调度
///
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let f1 = fs::read_to_string("./Cargo.toml");

    let f2 = fs::read_to_string("./Cargo.lock");


    // join try_join 是用来轮询多个 Future 的宏，它会依次处理每个 Future，遇到阻塞就处理下一个，直到所有 Future 生产结果
    let (content1, content2) = try_join!(f1, f2)?;

    let yaml1 = toml2yaml(&content1)?;
    let yaml2 = toml2yaml(&content2)?;


    let f3 = fs::write("/tmp/Cargo.yml", &yaml1);
    let f4 = fs::write("/tmp/Cargo.lock", &yaml2);

    try_join!(f3, f4)?;

    println!("{}", yaml1);
    println!("{}", yaml2);


    Ok(())
}

fn toml2yaml(content: &str) -> anyhow::Result<String> {
    let value: Value = toml::from_str(&content)?;
    Ok(serde_yaml::to_string(&value)?)
}