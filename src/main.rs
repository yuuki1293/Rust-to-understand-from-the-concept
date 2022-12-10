fn func_ex_print_some_match<T: std::fmt::Display>(ans: Option<T>) {
    match ans {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }
}
