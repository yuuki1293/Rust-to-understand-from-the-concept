enum DivError {
    DivByZero(i32),         // 0で割り算。i32は分子
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
        Err(DivError::DivByZero(a)) => {
            println!("{} divided by zero", a)
        }
        Err(DivError::BothNegative(a, b)) => {
            println!("Both numerator {} and denominator {} are negative", a, b) // numerator: 分子、denominator: 分母
        }
    }
}

fn main(){
    print_mydiv(5, 2);
    print_mydiv(-5, 0);
    print_mydiv(-5, -2);
}