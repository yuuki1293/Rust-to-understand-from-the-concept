struct GenEx<T> {
    value: T,
}

impl<T> GenEx<T> {
    fn return_value(self) -> T {
        self.value
    }
}

fn main() {
    let x1 = GenEx { value: 1 }; // x1はGenEx<i32>型と推論
    let x2 = GenEx {
        value: String::from("Hello"),
    }; // x2はGenEx<String>型と推論
    let x3 = GenEx::<f64> { value: 3.0 }; // 型パラメータを指定することもできる
    println!("x1: {}", x1.return_value());
    println!("x2: {}", x2.return_value());
    println!("x3: {}", x3.return_value());
}
