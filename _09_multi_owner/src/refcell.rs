use std::cell::{Ref, RefCell, RefMut};

fn main() {
    let data = RefCell::new(1);
    {
        let mut v: RefMut<i32> = data.borrow_mut();
        *v += 1;
    }

    let new_data: Ref<i32> = data.borrow();
    println!("data = {}", new_data);
}