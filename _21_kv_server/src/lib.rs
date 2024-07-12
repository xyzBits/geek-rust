/// 1. 最核心的功能是根据不同的命令进行诸如数据存贮，读取，监听等操作
/// 2. 而客户端要能通过网络访问 KV server，发送包含 命令的请求，得到结果
/// 3. 数据要能根据需要，存储在内存中或者持久化到磁盘上
///
/// 好的实现应该是在分析完需求后，首先从系统的主流程开始，搞清楚从客户端的请求到最终客户端收到响应，
/// 都会经过哪些主要的步骤，然后根据这些步骤，思考哪些东西需要延迟绑定，构建主要的接口和 trait
/// 等这些东西深思熟虑之后，再考虑实现，
///
/// client
/// IO tcp grpc http redis
/// SerDe Redis Json protobuf
/// 调度器 dispatcher
/// 处理逻辑 hset hmget sub
/// 存储 storage
/// 事件通知 logger wal stats
///
/// 1.  客户端 和服务器用什么协议通信，tcp http grpc http 支持一种还是多种
/// 2。 客户端 和服务器之间交互的应用层协议如何定义，怎么做序列化，反序列化，用 protobuf json redis resp ，或者也可以支持多种
/// 3。 服务器都支持哪些命令，第一版优先支持哪些
/// 4。 具体的处理逻辑中，要不要加 hook，在处理过程中发布一些事件，让其他流程可以得到通知，进行额外的处理，这些hook 可不可以提前终止整个流程的处理
/// 5。 在于存储，要支持不同的存储引擎么，MemDb RocksDb SledDb ，对于MemDb，我们考虑支持 WAL write ahead log 和 snapshot 吗
/// 6。 整个系统 可以配置吗，比如服务使用哪个端口，哪个存储引擎
///
///
///
/// 1。 像 KV server 这样需要高性能的场景，通信应该优先考虑 tcp，所以我们暂时只支持  tcp ，未来可以根据需要支持更多的协议，
/// 比如 http 2 grpc ，还有未来可能对安全性有额外的要求，可保证 TLS 这样的安全协议可以即插即用，总之，网络层需要灵活
/// 2。 应该层协议可以用 protobuf 定义，protobuf 直接解决了协议的定义，及如何序列化，反序列化，
/// 3。 服务器支持的命令可以参考 Redis 命令集，第一版支持 HXX 命令
/// 4。 处理流程中计划加这些 hook，收到客户端命令后 OnRequestReceived，处理完客户端的命令后 OnRequestExecuted
///    发送响应之前  BeforeResponseSend，发送响应之后  AfterResponseSend ，这样处理过程中的主要步骤都有事件暴露出去
///    让我们的KV server 可以非常灵活，方便调用者在初始化服务的时候注入额外的处理逻辑
///
/// 5。 存储必然需要足够灵活，可以对存储做个 trait 来抽象 其基本的行为，一开始可以只做 MemDB，未来肯定需要有支持持久化的存储
/// 6。 需要支持配置，但优先级不高，等基本流程搞定，使用过程中发现足够的痛点，就可以考虑配置文件如何处理了
///
/// 最重要的几个接口就是三个主体交互的接口，
///     客户端和服务器的接口或者说协议，
///     服务器和命令处理流程的接口
///     服务器和存储的接口
///
/// 客户端和服务器间的协议
///     首先是客户端和服务器之间的协议，用 protobuf 定义
///     通过 prost ，protobuf 可以被编译成 rust 代码，主要是 struct enum ，代我们使用，
///
/// CommandService trait
///     客户端和服务器间的协议敲定之后 ，就要思考如何处理请求的命令，返回响应
///     目前打算支持 9 种命令，未来可能支持更多命令，所以最好 定义一个 trait 来统一处理所有的命令，返回处理结果
///     在处理命令的时候，需要和存储发生关系，这样才能根据请求中携带的参数读取数据，或者把请求中的数据存入存储系统 中，
///     所以 trait 可以这样定义
/// ```Rust
/// pub trait CommandService {
///     fn execute(self, store: & impl Storage) -> CommandResponse;
/// }
/// ```
///
///     有了这个trait ，并且每一个命令都实现了这个trait 后，dispatch 方法就可以是类似这样的代码
///
///     这样，未来我们支持新命令时，只需要做两件事，为命令实现 CommandService，在dispatch 方法中添加新命令的支持
///
/// Storage trait
///     为不同的存储而设计的 storage trait ，提供 kv server 的主要接口
///     对于存储的抽象，不关心数据存在哪儿，但需要定义外界如何和存储打交道
///
///     从 CommandServiceTrait 中看到，在处理客户端 请求时，与之打交道的是 storage trait ，而不是
///     具体的某个 store，这样做的好处是，未来根据业务的需要，在不同场景下添加不同的 store，
///     只需要为其实现 storage trait 就可以，不必修改 commandService 有关的代码
///

mod service;
mod storage;
mod pb;
pub use pb::abi::*;