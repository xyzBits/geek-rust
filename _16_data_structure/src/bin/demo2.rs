use std::fmt::Debug;

fn main() {
    let mut v = vec![1, 2, 3, 4];
    let x: &[i32] = v.as_ref();

    let slice = &mut v[..2];
    slice[0] = 200;

    println!("{:?}", v);



}

fn print_slice<T: Debug>(s: &[T]) {
    println!("{:?}", s);
}


fn print_slice1<T, U>(s: T) where T: AsRef<U>, U: Debug {
    println!("{:?}", s.as_ref());

}