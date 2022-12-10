struct Person {
    name: String,
    age: u8,
}

impl Person {
    // 関連関数
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }
    // メソッド
    fn age_incr(&self, incr: u8) -> u8 {
        self.age + incr
    }
    // メソッド
    fn age_incr_replace(&mut self, incr: u8) {
        self.age += incr;
    }
}