use std::ops::Deref;

fn main() {
    let mut v1 = vec![1, 2, 3, 4];
    v1.push(5);

    println!("{}", v1.capacity());

    let b1: Box<[i32]> = v1.into_boxed_slice();
    let mut b2 = b1.clone();
    let v2 = b1.into_vec();
    println!("{}", v2.capacity());

    assert!(b2.deref() == v2);

    b2[0] = 2;
    println!("{:?}", b2);

    let b3 = Box::new([2, 22, 33, 22, 11, 55]);
    println!("{:?}", b3);

    
}