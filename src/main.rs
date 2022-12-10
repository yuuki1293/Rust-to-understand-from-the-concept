fn incr_by_owned(x: i32) -> i32 {
    x + 1
}

fn incr_by_ref(x_ref: &i32) -> i32 {
    *x_ref + 1
}

fn main() {
    let x = 1;
    let x_ref = &x;
    println!("{}", incr_by_owned(x));
    println!("{}", incr_by_ref(x_ref));
}