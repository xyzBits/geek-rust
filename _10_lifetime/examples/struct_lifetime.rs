// 只要为结构体中的每一个引用标注上生命周期
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str
        where 'a: 'b
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}


fn main() {
    let novel = String::from("Call me Ishmael. Some years age...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let important = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{:?}", important);


    let imp;
    {
        let hello = String::from("hello");
        imp = ImportantExcerpt {
            part: hello.as_str(),//结构体比字符串存活的久
        };
    }
    println!("{:?}", imp);
}