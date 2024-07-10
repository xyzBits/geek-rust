use std::collections::HashMap;

/// HashMap::new() 时，并没有分配空间，容量为 0 ，
/// 随着 hash table 不断被插入数据，会以 2^n-1 的方式增长，最小是 3
/// 当删除表中的数据时，原有的表大小不变，只有显式地调用 shrink_to_fit ，才会让 table 变小
/// 
fn main() {
    let mut map = HashMap::new();
    explain("empty", &map);

    map.insert('a', 1);
    explain("added 1", &map);

    map.insert('b', 2);
    map.insert('c', 3);
    explain("added 3", &map);

    map.insert('d', 4);
    explain("added 4", &map);


    // get 时需要使用引用，并且也返回引用
    assert_eq!(map.get(&'a'), Some(&1));
    assert_eq!(map.get_key_value(&'b'), Some((&'b', &2)));

    map.remove(&'a');
    // 删除后找不到了
    assert_eq!(map.contains_key(&'a'), false);
    assert_eq!(map.get(&'a'), None);
    explain("removed", &map);

    // shrink 后 hash table 变小
    map.shrink_to_fit();

    explain("shrink", &map);



}

fn explain<K, V>(name: &str, map: &HashMap<K, V>) {
    println!("name = {}, len = {}, cap = {}", name, map.len(), map.capacity());
}