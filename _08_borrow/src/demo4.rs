fn main() {
    let mut data = Vec::<u32>::new();

    // push_local_ref(&mut data);

    println!("{:?}", data);
}

// fn push_local_ref(data: &mut Vec<&u32>) {
//     // v does not live long enough
//     let v = 42;
//     data.push(&v);
// }