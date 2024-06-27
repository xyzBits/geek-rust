fn main() {
    // creates owned data from borrowed data, usually by cloning
    let hello = "hello world".to_owned();


    let data = &[1, 2];
    let vector = data.to_owned();

    // 并不改变原有的字符串，只是改变指向这个字符串的可变引用
    let mut temp = "hello";
    temp = "world";
    println!("{}", temp);




}