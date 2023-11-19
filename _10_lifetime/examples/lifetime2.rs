fn main() {
    let s1 = "abcd";
    let s2 = "xyz";

    let x = 4;
    let r1 = &x;
    // let r2 = &'ax;
    // let r3 = &'a mut x;

    let result = longest(s1, s2);
    println!("{}", result);


    let s1 = "hello".to_string();
    {
        let s2 = "world !!!".to_string();
        let result = longest(s1.as_str(), s2.as_str());
        println!("{}", result);
    }


    let s1 = "long string is long".to_string();
    let result;
    {
        let s2 = "short is short".to_string();
        result = longest(s1.as_str(), s2.as_str());
    }
    println!("{}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}