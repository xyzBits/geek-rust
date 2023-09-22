fn main() {
    let data = vec![10, 42, 3, 22, 12, 9, 4];
    let v = 34;
    let index_option = find_pos(data, v);

    if let Some(index) = index_option {
        println!("{}", index);
    } else {
        println!("nothing");
    }
}

fn find_pos(data: Vec<u32>, v: u32) -> Option<usize> {
    for (index, item) in data.iter().enumerate() {
        if *item == v {
            return Some(index);
        }
    }

    None
}