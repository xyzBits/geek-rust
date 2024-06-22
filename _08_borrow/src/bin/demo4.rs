fn main() {
    let mut data = Vec::new();

    let v = 42;

    data.push(&v);

    println!("{:?}", data);
}


/// 堆变量的生命周期不具备 任意长短的灵活性，因为堆上内存的生死存亡，
/// 跟栈上的所有者牢牢绑定，而栈上内存的生命周期，又跟栈的生命周期相关，
/// 所以我们只需要关心调用栈的生命周期
fn push_local_ref(data: &mut Vec<&u32>) {
    let v = 42;
    data.push(&v);
}