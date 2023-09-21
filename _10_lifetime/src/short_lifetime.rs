fn main() {
    let s1 = "Lindsey".to_string();
    let result;
    {
        // s2 does not live long enough
        let s2 = "Rosie".to_string();
        result = max(&s1, &s2);

        println!("{}", result);
    }// s2 dropped here while still borrowed

    // println!("bigger one: {}", result);// borrow later used here
}


fn max<'life>(s1: &'life str, s2: &'life str) -> &'life str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
}