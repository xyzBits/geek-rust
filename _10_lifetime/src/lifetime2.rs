fn main() {
    let s1 = "Lindsey";
    let s2 = "Rosie".to_string();

    let result = max(s1, &s2);

    println!("bigger one: {}", result);
}

fn max<'life>(s1: &'life str, s2: &'life str) -> &'life str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
}

// Parameter types contain explicit lifetimes that could be elided
// fn first<'life>(s: &'life str) -> &'life str {
fn first(s: &str) -> &str {
    let trimmed = s.trim();
    match trimmed.find(' ') {
        None => "",
        Some(pos) => &trimmed[..pos]
    }
}

#[test]
fn test_first_work() {
    let s1 = "hello world";
    let first = first(s1);
    println!("{}", first);
}
