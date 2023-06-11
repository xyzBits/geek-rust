fn main() {
    let data = vec![1, 2, 3, 4];
    let data1 = data;// move occurs because data has type ,
    // which does not implement the copy trait
    // println!("sum of data1: {}", sum(data1));
    // // borrow of moved value
    // println!("data1: {:?}", data1);//value borrow after move
    // println!("sum of data: {}", sum(data));
}

fn sum(data: Vec<u32>) -> u32 {
    data.iter().fold(0, |acc, x| acc + x)
}