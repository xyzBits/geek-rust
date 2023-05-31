fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("Fetching url: {}", url);

    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to markdown....");

    let md = html2md::parse_html(&body);
    // 我们使用了 unwrap() 方法，只关心成功返回的结果，结果出错，整个程序会终止
    // 如果想让错误传播，可以把所有的unwrap换成?操作符，并让main() 函数返回一个 Result<T, E>

    std::fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}", output);

    Ok(())
}
