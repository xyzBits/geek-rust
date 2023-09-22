fn main() {
    let mut data: Vec<&u32> = Vec::new();


    push_local_ref(&mut data);

    println!("{:?}", data);
}

fn push_local_ref(data: &mut Vec<&u32>) {
    let v = 42;
    // data.push(&v);
    //v does not live enough

    // dropped here while still borrowed
}