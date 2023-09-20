use std::mem;

fn main() {
    let mut data = vec![1, 2, 3];
    let data1 = vec![&data[0]];
    println!("{}", &data[0]);

    // for i in 0..100 {
    //     data.push(i);
    // }

    println!("{}", &data[0]);
    println!("{:?}", &data1);
}

/// 可变引用与不可变引用不能共存的示例
#[test]
fn test1() {
    let mut arr = vec![1, 2, 3];
    let first = arr.first();
    println!("first ref = {:p}", &first);

    arr.push(4);
    (1..200).into_iter().for_each(|i| arr.push(i));

    let first = arr.first();
    println!("first ref = {:p}", &first);// 在push 的时候，重新分配了内存，如果last 继续保留，会读到已经释放的内存



    // println!("{:?}", last);

    println!("{:?}", arr);
}

#[test]
fn test2() {
    let mut v = vec![1];
    let v1 = Vec::<i32>::with_capacity(6);
    println!("v1: {:?}", v1);

    println!("heap start: {:p}", &v[0] as *const i32);

    extend_vec(&mut v);

    println!("new heap start: {:p}", &v[0] as *const i32);

    print_vec("v", v);
}


fn extend_vec(v: &mut Vec<i32>) {
    // Vec<T> 堆内存里 T 的个数是指数增长的，我们恰好让它 push 33 个元素
    // capacity 会变成 64
    (2..34).into_iter().for_each(|i| v.push(i));
}

fn print_vec<T>(name: &str, data: Vec<T>) {
    let p: [usize; 3] = unsafe {mem::transmute(data)};
    print!("{}: 0x {:x}, {}, {}", name, p[0], p[1], p[2]);
}