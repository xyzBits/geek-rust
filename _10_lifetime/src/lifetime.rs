fn main() {
    let s1 = "Lindsey".to_string();
    let s2 = "Rosie".to_string();

    let result = max(&s1, &s2);
    println!("bigger one: {}", result);

    let result = get_max(&s1);
    println!("bigger one: {}", result);

}

fn get_max(s1: &str) -> &str {
    max(s1, "Cynthia")
}

fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
}

