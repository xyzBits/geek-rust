use std::cell::UnsafeCell;
use std::mem;

fn main() {
    let mut v = vec![1];

    let v1: Vec<i32> = Vec::with_capacity(9);

    print_vec("v1", v1);

    println!("heap start: {:p}", &v[0] as * const i32);

    extend_vec(&mut v);

    println!("heap start: {:p}", &v[0] as * const i32);


    print_vec("v", v);

}

fn extend_vec(v: &mut Vec<i32>) {
    (2..34).into_iter().for_each(|i| v.push(i));
}

fn print_vec<T>(name: &str, data: Vec<T>) {
    let p: [usize; 3] = unsafe {
        mem::transmute(data)
    };

    println!("{}: 0x{:x}, {}, {}", name, p[0], p[1], p[2])
}