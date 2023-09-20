fn main() {
    types_impl_copy_trait();
}

fn is_copy<T: Copy>() {

}

fn types_impl_copy_trait() {
    is_copy::<bool>();
    is_copy::<char>();

    is_copy::<u8>();
    is_copy::<u16>();
    is_copy::<u32>();
    is_copy::<u64>();
    is_copy::<usize>();

    is_copy::<f32>();
    is_copy::<f64>();

    // 函数指针是 copy
    is_copy::<fn()>();

    is_copy::<*const String>();
    is_copy::<*mut String>();

    // 不可变引用是 copy
    is_copy::<&[Vec<u8>]>();
    is_copy::<&String>();

    // 数组，元组，如果其内部类型是 copy ，那么它们也是 copy
    is_copy::<[u8; 4]>();
    is_copy::<(&str, &str)>();

}

fn types_not_impl_copy_trait() {
    // DST 类型不是 copy
    // is_copy::<str>();
    // is_copy::<[u8]>();

    // is_copy::<Vec<u8>>();
    // is_copy::<String>();

    // is_copy::<&mut String>();

    // is_copy::<[Vec<u8>; 4]>();
    // is_copy::<(String, u32)>();
}