trait Print {
    fn print(self);
}

// impl <'a> Print for &'a str {
//     fn print(self) {
//         println!("Arbitrary str: {}", self);
//     }
// }

impl Print for &'static str {
    fn print(self) {
        println!("static str: {}", self);
    }
}

fn main() {
    let s = "hello, world";
    s.print();
}