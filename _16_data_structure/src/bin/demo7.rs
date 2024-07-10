fn main() {
    let arr = ['h', 'e', 'l', 'l', 'o'];
    let vec = vec!['h', 'e', 'l', 'l', 'o'];

    let s = String::from("hello");
    let s1 = &arr[1..3];
    let s2 = &vec[1..3];

    let s3 = &s[1..3];
    println!("{:?}", s1);
    println!("{:?}", s2);
    println!("{:?}", s3);

    // 是否相等取决于 长度和内容 是否相等
    assert_eq!(s1, s2);

    assert_eq!(s2, s3.chars().collect::<Vec<_>>());

    // &[char] 可以通过迭代器转换成 String， String 可以和 &str 直接对比
    assert_eq!(String::from_iter(s2), s3);
}