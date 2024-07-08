use std::borrow::Cow;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct User<'input> {
    #[serde[borrow]]
    name: Cow<'input, str>,

    age: u8,
}

fn main() {
    let input = r#"{"name": "bitch", "age": 19}"#;

    let user: User = serde_json::from_str(input).unwrap();

    match user.name {
        Cow::Borrowed(x) => { println!("borrowed {}", x) }
        Cow::Owned(x) => { println!("owned {}", x) }
    }
}