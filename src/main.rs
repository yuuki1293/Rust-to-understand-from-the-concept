use thiserror::Error;

#[derive(Error, Debug)]
enum DivError {
    #[error("{0} divided by zero")]
    DivByZero(i32), // 0で割り算。i32は分子
    #[error("Both numerator {0} and denominatror {1} are negative")]
    BothNegative(i32, i32), // 分子、分母ともに負の数。2つのi32はそれぞれ分子、分母
}

fn mydiv(x: i32, y: i32) -> Result<i32, DivError> {
    if y == 0 {
        Err(DivError::DivByZero(x))
    } else if x < 0 && y < 0 {
        Err(DivError::BothNegative(x, y))
    } else {
        Ok(x / y)
    }
}

fn print_mydiv(x: i32, y: i32) {
    match mydiv(x, y) {
        Ok(ans) => println!("no error. ans = {}", ans),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    print_mydiv(5, 2);
    print_mydiv(-5, 0);
    print_mydiv(-5, -2);
}
