use clap::Parser;
// 
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "dong fang")]
struct Opts {
    #[clap(subcommand)]
    sub_cmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post)
}

#[derive(Parser, Debug)]
struct Get {
    url: String,
}

#[derive(Parser, Debug)]
struct Post {
    url: String,
    body: Vec<String>
}

///cargo run --example cli  post httpbin.org/post a=1 b=2
fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);


}