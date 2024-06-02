use std::future::Future;
use futures::executor::block_on;

#[tokio::main]
async fn main() {
    let name1 = "tom".to_string();
    let name2 = "jerry".to_string();

    say_hello1(&name1).await;
    say_hello2(&name2).await;

    block_on(say_hello1(&name1));
    block_on(say_hello2(&name2));
}

async fn say_hello1(name: &str) -> usize {
    println!("Hello {name}");
    42
}


#[allow(clippy::all)]
fn say_hello2<'fut>(name: &'fut str) -> impl Future<Output = usize> + 'fut {
    async move {
        println!("Hello {name}");
        42
    }
}