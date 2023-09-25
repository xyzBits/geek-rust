fn main() {
    let arr = ['h', 'e', 'l', 'l', 'o'];
    let vec = vec!['h', 'e', 'l', 'l', 'o'];
    let s = "hello".to_string();

    let s1 = &arr[..2];
    let s2 = &vec[..2];
    let s3 = &s[..2];
    println!("s1: {:?}", s1);
    println!("s2: {:?}", s2);
    println!("s3: {:?}", s3);

    assert_eq!(s1, s2);
    assert_eq!(s2, s3.chars().collect::<Vec<_>>());

    // &[char] 可以通过迭代器转换成 String ,String 可以和 &str 直接 对比
    assert_eq!(String::from_iter(s2), s3);
}