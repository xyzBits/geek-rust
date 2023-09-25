
pub trait TypeName {
    fn type_name(&self) -> &'static str;
}

impl <T> TypeName for T {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

fn main() {
    let s = "hello".to_string();
    let s1 = &s;
    let s2 = s.as_str();
    let s3 = &s[..];
    println!("s: {}", s.type_name());
    println!("s1: {}", s1.type_name());
    println!("s2: {}", s2.type_name());
    println!("s3: {}", s3.type_name());

    let v = vec![1, 2, 3, 4];
    let v1 = &v;
    let v2 = v.as_slice();
    let v3 = &v[..];
    let v4 = v.clone().into_boxed_slice();
    println!("v: {}", v.type_name());
    println!("v1: {}", v1.type_name());
    println!("v2: {}", v2.type_name());
    println!("v3: {}", v3.type_name());
    println!("v4: {}", v4.type_name());
}