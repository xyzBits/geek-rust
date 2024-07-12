
/// 要创建 src/pb 目录，否则 编译不过，
/// 在项目根目录下执行 cargo build 会生成 src/pb/abi.rs 文件，
/// 里面包含所有 protobuf 定义的消息的 Rust 数据结构，
///
fn main() {
    let mut config = prost_build::Config::new();

    // 这里我们为编译出来的代码额外添加了一些属性，比如为 protobuf 的 bytes 类型生成 Bytes
    // 而非默认的 Vec<[u8]> 为所有类型加入 PartialOrd 派生宏

    // configure the code generator to generate Rust bytes::Bytes fields for Protobuf bytes type fields
    config.bytes(&["."]);


    config.type_attribute(".", "#[derive(PartialOrd)]");
    config
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap()
}