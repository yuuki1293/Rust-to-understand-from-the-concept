fn generic_return_type<T: std::fmt::Display>(x: T) -> T {
    x
}

fn main() {
    println!("{}", generic_return_type(1));
    println!("{}", generic_return_type("Hello"));
}
