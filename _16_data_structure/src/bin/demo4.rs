use itertools::Itertools;

fn main() {
    let err_str = "bad happened";
    let input = vec![Ok(21), Err(err_str), Ok(7)];

    let ok = input
        .into_iter()
        .filter_map_ok(|item| {
            if item > 10 {
                Some(item * 2)
            } else { None }
        })
        .collect::<Vec<_>>();

    println!("{:?}", ok);
}