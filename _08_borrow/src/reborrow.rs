fn main() {
    println!("Hello, world!");

    let mut data = vec![1, 2, 3, 4];
}


fn sum(data: &mut Vec<i32>) -> i32 {
    println!("addr of the ref v: {:p}", &data);
    data.iter().sum()
}