fn fib_loop(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2u8);
    println!("\n fib_loop");

    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next val is {}", b);
        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2u8);
    println!("\n fib_while");

    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        println!("next val is {}", b);
    }
}

fn fib_for(n: u8) {
    let (mut a, mut b, mut c) = (1, 1, 2u8);
    println!("\n fib_for");

    for _i in 2..n {//不能使用负数，因为上下标是usize类型，不能为负数
        let c = a + b;
        a = b;
        b = c;
        println!("next val is {}", b);
    }
}

fn main() {
    let n = 10u8;
    fib_loop(n);
    fib_while(n);
    fib_for(n);
}