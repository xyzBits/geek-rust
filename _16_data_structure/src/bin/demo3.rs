fn main() {}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    #[test]
    fn test_1() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }


    #[test]
    fn test_2() {
        let x = Box::new(1);
        // 智能指针被 * 解引用为 i32 类型
        let sum = *x + 1;
    }

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        // 返回的是一个常规引用，可以被 * 进行解引用
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[test]
    fn test_3() {
        let y = MyBox::new(5);
        assert_eq!(5, *y);
    }


    fn display(s: &str) {
        println!("{}", s);
    }

    #[test]
    fn test_4() {
        let s = String::from("hello world");
        /// String 实现了 deref trait ，可以在需要时自动被转换为 &str
        /// &s 是一个 &String  类型，当它被传给 display 函数时，自动通过 Deref 转换成了 &str
        /// 必须 使用 &s 的方式来触发 Deref，仅引用类型的实参才会触发自动解引用，
        let x = &s;
        display(x);
    }


    #[test]
    fn test_5() {
        let my_box = MyBox::new(String::from("hello"));
        let s1: &str = &my_box;
        display(s1);

        let s2 = &my_box;
        // myBox 根本没有 to_string 方法，完全是因为编译器对 myBox 应用了 Deref 的结果，
        // 方法调用会自动解引用
        let string = s2.to_string();
    }
}