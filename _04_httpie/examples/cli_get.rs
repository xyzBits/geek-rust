use std::collections::HashMap;
use std::str::FromStr;
use anyhow::{anyhow, Result};
use clap::{Parser};
use reqwest::{Client, Url};

#[derive(Debug, Parser)]
#[clap(version = "1.0", author = "dong fang")]
struct Opts {
    #[clap(subcommand)]
    sub_cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

#[derive(Debug, Parser)]
struct Get {
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}


#[derive(Debug, Parser)]
struct Post {
    #[clap(parse(try_from_str = parse_url))]
    url: String,

    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}

// 命令行中的 key=vale 可以通过 parse_kv_pair 解析成KvPair结构
#[derive(Debug, PartialEq, Eq)]
struct KvPair {
    k: String,
    v: String,
}

// 当我们实现 FromStr trait 后，可以使用 str.parse() 方法将字符串解析 成 KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 使用 = 进行split时，会得到一个迭代器
        // let mut split = s.split('=');

        let err = || anyhow!(format!("Failed to parse {}", s));
        let (k, v) = s.split_once('=').ok_or_else(err)?;

        // KvPair也可以换成 Self
        Ok(Self {
            // k: (split.next().ok_or_else(err)?).to_string(),
            k: k.trim().to_string(),
            // v: (split.next().ok_or_else(err)?).to_string(),
            v: v.trim().to_string(),
        })
    }
}

// 因为我们为 kvPair 实现了 FromStr ，这里可以直接 s.parse() 得到 kvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    // s.parse::<KvPair>()
    s.parse()// 类型从 返回值类型推导得出
}


fn parse_url(s: &str) -> Result<String> {
    //这里仅检查一下 url 是否合法
    //Url也实现了 str::FromStr 这个trait
    let _url: Url = s.parse()?;
    let _url = s.parse::<Url>()?;

    Ok(s.into())
}


async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    println!("{}", resp.text().await?);
    Ok(())
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();

    for pair in args.body.iter() {
        // body.insert(pair.k, pair.v); // cannot move
        body.insert(&pair.k, &pair.v);
    }

    let resp = client.post(&args.url).json(&body).send().await?;
    println!("{}", resp.text().await?);
    Ok(())
}


///
/// cargo run --example cli_get  post https://httpbin.org/post a=1 b=2
///
#[tokio::main]
async fn main() -> Result<()> {

    // let expected = KvPair { k: "a".to_string(), v: "1".to_string() };
    //
    // let result = "a = 1".parse::<KvPair>().unwrap();
    // println!("{:?}", result);
    // assert_eq!(result, expected);
    //
    // let result = parse_kv_pair("a =  b");
    // println!("{:?}", result);

    let opts = Opts::parse();

    let client = Client::new();

    match opts.sub_cmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_hash_map_works() {
        let mut map = HashMap::new();

        let mut  key = String::from("hello");
        let value = key.len();
        map.insert(&key, value);
        println!("{:?}", map);

        key.push_str(" world");
        println!("{}", key);// borrow of moved value key
    }
}