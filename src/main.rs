fn func_ex_print_some<T: std::fmt::Display>(ans: Option<T>) {
    if ans.is_some() {
        println!("{}", ans.unwrap())
    } else {
        println!("None")
    }
}
