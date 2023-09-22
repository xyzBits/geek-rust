fn main() {
    let data = vec![1, 2, 3, 4];

    let data1 = &data;


    let data2 = &data;
    let data3 = &data;
    println!("{:p}", &data);
    println!("{:p}", data1);
    println!("{:p}", data2);
    println!("{:p}", data3);
    println!("==============================================");

    println!("{:p}", &&data);
    println!("{:p}", &data1);
    println!("{:p}", &data2);
    println!("{:p}", &data3);
    println!("==============================================");
    // 值的地址是什么？引用的地址又是什么？
    println!(
        "addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}",
        &data, data1, &&data, &data1
    );
    println!("sum of data1: {}", sum(data1));

    // 堆上数据的地址是什么？
    println!(
        "addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );
}

fn sum(data: &[u32]) -> u32 {
    println!("addr of the value: {:p}, addr of ref {:p}", data, &data);
    data.iter().sum()
}