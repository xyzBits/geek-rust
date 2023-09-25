use std::fmt::Debug;

fn main() {
    let v = vec![1, 2, 3, 4];
    let v1 = &v;
    let v2 = v.as_slice();
    let v3 = &v[..];

    let v = vec![1, 2, 3, 4];
    let v_ref = &v;
    let v_slice = &v[..];

    // vec 实现了 deref &Vec[T] 会自动 被解引用 为 &[T]
    print_slice(&v);

    print_slice(&v[..]);

    print_slice1(&v);
    print_slice1(&v[..]);

    print_slice1(v);

    let arr = [1, 2, 3, 4];
    let arr_ref = &arr;
    let arr_slice = &arr[..];
    print_slice(&arr);
    print_slice(&arr[..]);
    print_slice1(&arr);
    print_slice1(&arr[..]);


    let vec = vec![2, 1, 11, 1, 4];
    print_slice(&vec);
}

fn print_slice<T: Debug>(s: &[T]) {
    println!("{:?}", s);
}

fn print_slice1<T, U>(s: T)
    where T: AsRef<[U]>, U: Debug,
{
    println!("{:?}", s.as_ref());
}