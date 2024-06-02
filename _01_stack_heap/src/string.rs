fn main() {
    let data: String = "hello".into();

    let s1 = &data;
    let s2 = &data;
    let s3 = &data;

    dbg!(&s1 as *const _);
}