use std::future::Future;

use futures::executor::block_on;

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