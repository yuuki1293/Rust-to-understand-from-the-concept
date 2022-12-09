fn myprint<T: std::fmt::Display>(msg: T) {
    println!("{}", msg);
}

fn main() {
    let s = "Hello".to_string();
    myprint(s); // sの所有権が関数内の変数に移動
    myprint(s); // sの所有権は移動してしまい初期化されていない変数になるのでエラーになる
}