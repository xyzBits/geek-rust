fn main() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";

    println!("Fetching url: {}", url);

    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown....");

    let md = html2md::parse_html(&body);
    // 我们使用了 unwrap() 方法，只关心成功返回的结果，结果出错，整个程序会终止

    std::fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}", output);
}
