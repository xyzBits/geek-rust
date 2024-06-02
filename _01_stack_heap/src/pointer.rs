use std::cmp::max;

static MAX: u32 = 0;

fn foo() {

}

fn main() {
    let hello = "hello world".to_string();

    let data = Box::new(1);

    println!("row data: {:p}", "hello world");

    println!("data (static var): {:p}", &&MAX);
    println!("{:p}", &MAX);
    println!("{:p}", &&MAX);
    println!("{:p}", &&MAX);

}