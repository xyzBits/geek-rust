///cargo run --example cli  post httpbin.org/post a=1 b=2
fn main() {
    let args = std::env::args();

    //cargo run --example args hello world
    //Args { inner: ["target/debug/examples/args", "hello", "world"] }
    println!("{:?}", args);

    let args = args.skip(1).collect::<Vec<_>>();


    //["hello", "world"]
    //去掉前面的 target/debug/examples/args
    println!("{:?}", args);

    let args = std::env::args();
    let v = args.collect();
    println!("{:?}", v);
}