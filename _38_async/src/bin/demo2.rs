use std::future::Future;

use futures::executor::block_on;
/// async fn 的返回值，是一个 impl Future，
/// 如果给一个普通函数返回 impl Future<Output> ，行为是否和 async fn 是一样的呢
/// 两个是等价的，
/// 二者都可以使用 await 来执行，也可以将其提供给一个 executor 来执行
#[tokio::main]
async fn main() {
    let name1 = "bitch".to_string();
    let name2 = "fuck".to_string();


    say_hello_async(&name1).await;
    say_hello_return_async(&name2).await;

    block_on(say_hello_async(&name1));
    block_on(say_hello_return_async(&name2));
}

async fn say_hello_async(name: &str) -> usize {
    println!("Hello {}", name);
    42
}

/// async fn 关键字相当于返回一个 impl Future<Output> 的语法糖
fn say_hello_return_async<'fut>(name: &'fut str) -> impl Future<Output=usize> + 'fut {
    async move {
        println!("Hello {}", name);
        42
    }
}