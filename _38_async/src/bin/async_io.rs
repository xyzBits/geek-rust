use toml::Value;
use anyhow::Result;
use tokio::{fs, try_join};

#[tokio::main]
async fn main() -> Result<()> {
    let f1 = fs::read_to_string("./Cargo.toml");
    let f2 = fs::read_to_string("./Cargo.lock");

    // try_join! 是用来轮询多个 future 的宏，它会依次处理每个 future，遇到阻塞就处理下一个，直到所有的future产生结果

    let (content1, content2) = try_join!(f1, f2)?;

    // 计算
    let yaml1 = toml2yaml(&content1)?;
    let yaml2 = toml2yaml(&content2)?;

    // 写入 /tmp/Cargo.yml，IO 操作 3
    let f3 = fs::write("/tmp/Cargo.yml", &yaml1);
    // 写入 /tmp/Cargo.lock，IO 操作 4
    let f4 = fs::write("/tmp/Cargo.lock", &yaml2);
    try_join!(f3, f4)?;

    // 打印
    println!("{}", yaml1);
    println!("{}", yaml2);

    Ok(())

}


fn toml2yaml(content: &str) -> anyhow::Result<String> {
    let value = toml::from_str::<Value>(content)?;
    Ok(serde_yaml::to_string(&value)?)
}