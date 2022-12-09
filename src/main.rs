fn myprint<T: std::fmt::Display>(msg: T) {
    println!("{}", msg);
}

fn main() {
    let s = "Hello".to_string();
    let ss = s.clone(); // sのコピーをssに作っておく
    myprint(s); // sの所有権が関数内の変数に移動
    myprint(ss); // ssの所有権が関数内の変数に移動
}