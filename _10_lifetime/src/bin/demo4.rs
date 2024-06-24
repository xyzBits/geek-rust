fn main() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();

    let hello = strtok(&mut s1, ' ');

    println!("hello = {}", hello);
    println!("s1 = {}", s1);
    println!("s = {}", s);


}

pub fn strtok<'b, 'a>(s: &'b mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}