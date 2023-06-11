fn main() {
    let mut data: Vec<&u32> = Vec::new();

    let v = 43;

    data.push(&v);

    println!("{:?}", data);
}