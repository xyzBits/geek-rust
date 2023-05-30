use std::collections::HashMap;
use std::str::FromStr;
use clap::{Parser, AppSettings};
use anyhow::{anyhow, Result};
use reqwest::{Url, header, Client, Response};
use colored::*;


// 子命令分别对应不同的 http 方法，目前只支持 get / post
#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

// feed get with an url and we will retrieve the response for you
#[derive(Parser, Debug)]
struct Get {
    // http 请求的url
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

// post 子命令，需要输入一个url，和若干个可行的 key-value 用于提供 json body
// feed post with an url and optional key=value pairs, we will post the data
// as JSON, and retrieve the response for you
#[derive(Parser, Debug)]
struct Post {
    // http 请求的url
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    // http 请求的body
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}

// 命令行中的key=value，可以通过 parse_kv_pair 解析成 KvPair 结构
#[derive(Debug, PartialEq)]
struct KvPair {
    k: String,
    v: String,
}

// 当我们实现 FromStr trait 后，可以用 str.parse() 方法将字符串解析成 KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // 使用 = 进行split，这会得到一个迭代器
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            //迭代器中取第一个结果作为key，迭代器返回 Some(T)/ None
            // 我们将其中转换成Ok(T)/Err(e) 然后用 ? 处理错误
            k: (split.next().ok_or_else(err)?).to_string(),
            // 从迭代器中取第二个结果作为value
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

// 定义httpie 的cli的主入口，它包含若干个子命令
// /// 是注释文档，clap 会将其作为cli的帮助
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "dong fang")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

fn parse_url(s: &str) -> Result<String> {
    // 检查一下url是否合法
    let _url: Url = s.parse()?;

    Ok(s.into())
}


async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    println!("{:?}", resp.text().await?);
    Ok(())
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    println!("{:?}", resp.text().await?);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);

    //生成一个 http 客户端
    let client = Client::new();

    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };

    Ok(result)
}

//仅在 cargo test 时才编译
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.xyz").is_ok());
        assert!(parse_url("https://httpbin.org/post").is_ok());
    }

    #[test]
    fn parse_kv_pair_work() {
        assert!(parse_kv_pair("a").is_err());
        assert_eq!(parse_kv_pair("a=1").unwrap(),
                   KvPair { k: "a".into(), v: "1".into() });

        assert_eq!(parse_kv_pair("b=").unwrap(),
                   KvPair { k: "b".into(), v: "".into() });
    }
}