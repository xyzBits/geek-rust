fn main() {
    let data = vec![1, 2, 3, 4];
    let data1 = data;
    println!("sum of data1 {}", sum(data1));

    // 无法编译通过
    // println!("{}", sum(data));
    // println!("{:?}", data1);
}


fn sum(data: Vec<u32>) -> u32 {
    data.iter().sum()
}