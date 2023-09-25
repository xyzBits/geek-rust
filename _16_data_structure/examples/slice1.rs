
// 可以将切片看成一种工具，让我们可以统一访问行相同，结构类似但有些许差异的类型
// 对于array 和 vecotr，虽然是不同的数据结构，一个放在栈上，一个放在堆上，但它们的切片
// 是类似的，而且对于相同内容数据的相同切片
// &arr[1..3] &vec[1..3] 这两者是等价的，除此之外，切片和对应的数据结构也可以直接比较

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let vec = vec![1, 2, 3, 4, 5];


    let s1 = &arr[1..3];
    let s2 = &vec[1..3];
    println!("s1: {:?} s2: {:?}", s1, s2);

    // &[T] 和 &[T] 是否相等取决于长度和内容是否相等
    assert_eq!(s1, s2);

    // &[T] 可以和 Vec<T> 和 T[;n] 比较，也会看长度的内容
    assert_eq!(&arr[..], vec);
    assert_eq!(&vec[..], arr);
}