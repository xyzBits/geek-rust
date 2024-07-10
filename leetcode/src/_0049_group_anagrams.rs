use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();

        for item in strs {
            let mut chars = item.chars().collect::<Vec<_>>();
            chars.sort();
            let key = chars.iter().collect::<String>();
            map.entry(key).or_insert(vec![]).push(item);
        }

        map.into_values().collect::<Vec<Vec<_>>>()
    }
}

#[test]
fn test_sort_chars() {
    let data = "hello world, a baby";

    let mut chars = data.chars().collect::<Vec<_>>();
    chars.sort();
    // chars.sort_by(|a, b| a.cmp(b));


    println!("{:?}", chars);
}

#[test]
fn test_map_entry() {
    let mut map = HashMap::new();

    map.insert("hello".to_owned(), vec!['h', 'e', 'l', 'o']);

    map
        .entry("hello".to_owned())
        .or_insert(vec![])
        .append( &mut vec!['w', 'o', 'r', 'l', 'd']);


}