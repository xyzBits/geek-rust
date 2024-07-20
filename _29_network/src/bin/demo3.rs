/// 如何处理大量的连接
///     线程切换有一个 save and load 的上下文切换过程
///     如果使用线程，遇到C10K 瓶颈，也就是连接达到万级别，系统就会遭遇资源和算力的双重瓶颈
///     Rust 中栈的默认是 2M,10k需要20G的内存
///     从算力看，太多线程在连接数据到达时，会来回切换线程，导致CPU过分忙碌，无法处理更多的连接请求
///
///     所以，对于有大量连接的网络服务时，使用线程不是一个好的方式
///     如果要突破 C10K 的瓶颈，达到 C10M，只能在用户态使用协程来处理
///     Rust 是无栈协程
///
/// 如何处理信息共享
///     如果不修改，使用 Arc<T>
///     如果需要修改，使用 Arc<RwLock<T>>
///     使用锁，就意味着一旦在关键路径上需要访问被锁住的资源 ，整个系统的吞吐量就会受到很大的影响
///     一种思路是，把锁的粒度降低，这样冲突就会减少，比如kv server中，key 的hash 模一下N，
///     将不同的 key 分到 N 个 memory store 中，这样，锁的粒度就降到了 1/N
///
///     另一种思路是改变资源共享的方式，使其只被一个特定的线程访问，
///     其他线程或者协程只能通过给其他发消息的方式与之交互
///     也就 是channel
///
///
///  处理网络数据的一般方法
///     如何处理网络数据，大部分时候，使用已有的应用层协议来处理网络数据，例如HTTP
///     在 HTTP 协议下，基本上使用 JSON 构建  REST API / JSON API 是业界常识
///     客户端和服务端也有足够好的生态来支持这样的处理，
///     你只需要使用 serde 让你定义的 Rust 数据结构 具备 Serialize / Deserialize 的能力
///     然后用 serde_json 生成序列化后的json数据
#[macro_use]
extern crate rocket;

use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Hello {
    name: String,
}


#[get("/", format = "json")]
fn hello() -> Json<Hello> {
    Json(Hello {
        name: "bitch".into()
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
