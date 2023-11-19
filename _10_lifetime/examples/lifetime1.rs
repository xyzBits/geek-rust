fn main() {
    let r;
    {
        let x = 5;// binding x declared here
        r = &x;// x does not live long enough
    }// x dropped here while still borrowed
    // 当 x 被释放后，r所引用的值就不再是合法的，会导致我们的程序发生异常行为
    // 且异常行为有时候会很难被发现

    println!("{}", r);// borrow later used here
}