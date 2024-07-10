use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

/// 让自定义的数据结构 做 Hash Key
/// 有时候，需要让自定义的数据结构成为 HashMap 的 key，
/// 此时，要用到三个 trait Hash PartialEq, Eq
///
/// Hash ，可以让数据结构计算 哈希
/// 实现了 PartialEq, Eq 可以让数据结构进行相等和不相等的比较，
/// Eq 实现了比较的自反性，a == b
/// 对称性 a == b 则 b == a
/// 以及传递性 a == b, b == c 则 a == c
/// PartialEq 没有实现自反性


// 如果要支持 Hash，可以用 #[derive(Hash)] 前提是每个字段都实现了 Hash
// 如果要能作为 HashMap 的 Key，还需要 PartialEq 和 Eq
#[derive(Debug, Hash, PartialEq, Eq)]
struct Student<'a> {
    name: &'a str,
    age: u8,
}

impl<'a> Student<'a> {
    pub fn new(name: &'a str, age: u8) -> Self {
        Self {
            name,
            age,
        }
    }
}

fn main() {
    // let mut hasher = DefaultHasher::new();
    let student = Student::new("tom", 29);

    // 实现了 Hash 的数据结构可以 直接调用 hash方法
    // student.hash(&mut hasher);

    let mut map = HashMap::new();

    map.insert(student, vec!["Match", "Writing"]);

    // println!("hash: 0x{:x}, map: {:?}", hasher.finish(), map);

    println!("{:?}", map);
}