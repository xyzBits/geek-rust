use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use clap::Parser;
use colored::Colorize;
use mime::Mime;
use reqwest::{Client, header, Response, Url};
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};

/// 以下部分用于处理 cli
/// 定义 httpie 的 cli 主入口，它包含若干个子命令
///
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

/// 处理 get 子命令
async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    // println!("{}", resp.text().await?);
    Ok(print_resp(resp).await?)
}

/// 处理 post 子命令
async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();

    for pair in args.body.iter() {
        // body.insert(pair.k, pair.v); // cannot move
        body.insert(&pair.k, &pair.v);
    }

    let resp = client.post(&args.url).json(&body).send().await?;
    // println!("{}", resp.text().await?);
    Ok(print_resp(resp).await?)
}

/// 打印服务器版本号 + 状态码
fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

/// 打印服务器返回的 http header
fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    println!();
}


/// 打印服务器返回的 http body
fn print_body(m: Option<Mime>, body: &str) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => print_syntect(body, "json"),
        Some(v) if v == mime::TEXT_HTML => print_syntect(body, "html"),
        _ => println!("{}", body),
    }
}

/// 将服务器返回的 content-type 解析成 Mime 类型
fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);

    let mime = get_content_type(&resp);
    let body = resp.text().await?;

    print_body(mime, &body);

    Ok(())
}

fn print_syntect(s: &str, ext: &str) {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension(ext).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(s) {
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
}


///
/// cargo run --example cli_get  post https://httpbin.org/post a=1 b=2
/// 程序的入口函数，因为在 http 请求时，我们使用了异步处理，所以这里引入 tokio
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

    let mut headers = header::HeaderMap::new();

    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);


    let client = Client::builder()
        .default_headers(headers)
        .build()?;

    match opts.sub_cmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{KvPair, parse_kv_pair, parse_url};

    #[test]
    fn test_hash_map_works() {
        let mut map = HashMap::new();

        let mut key = String::from("hello");
        let value = key.len();
        map.insert(&key, value);
        println!("{:?}", map);

        key.push_str(" world");
        println!("{}", key);// borrow of moved value key
    }

    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc-hello.xyz").is_ok());
        assert!(parse_url("https://httpbin.org/post").is_ok());
    }

    #[test]
    fn parse_key_pair_works() {
        assert!(parse_kv_pair("a").is_err());

        assert_eq!(
            parse_kv_pair("a=1").unwrap(),
            KvPair {
                k: "a".into(),
                v: "1".into(),
            }
        );

        assert_eq!(
            parse_kv_pair("b=").unwrap(),
            KvPair {
                k: "b".into(),
                v: "".into(),
            }
        );
    }
}