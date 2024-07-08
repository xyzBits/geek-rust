use std::borrow::Cow;

use url::Url;

/// Cow
///
fn main() {
    let url = Url::parse("https://tyr.com/rust?page=1024&sort=desc&extra=hello%20world").unwrap();

    let mut paris = url.query_pairs();

    assert_eq!(paris.count(), 3);

    let (mut key, value) = paris.next().unwrap();
    // 因为 key value 都是 Cow<str> 他们用起来感觉和 &str 或者 String 一样
    // 此刻，他们都是 borrowed
    println!("key: {}, value: {}", key, value);

    key.to_mut().push_str("_lala");

    print_pairs((key, value));

    print_pairs(paris.next().unwrap());

    // 在处理 extra=hello%20world 时，value 被处理成 hello world
    // 所以这里 value 是 owned
    print_pairs(paris.next().unwrap());
}

fn print_pairs(pair: (Cow<str>, Cow<str>)) {
    println!("key: {}, value: {}", show_cow(pair.0), show_cow(pair.1));
}

fn show_cow(cow: Cow<str>) -> String {
    match cow {
        Cow::Borrowed(value) => { format!("Borrowed {}", value) }
        Cow::Owned(value) => { format!("Owned {}", value) }
    }
}