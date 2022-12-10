use std::{
    fs::File,
    io::{BufRead, BufReader},
};
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("./input.txt")?;
    let f = BufReader::new(f);

    for line in f.lines().flatten() {
        let mut v = Vec::new();
        for ee in line.split(' ') {
            v.push(ee.parse()?);
        }
        let result = mydiv(v[0], v[1])?;
        println!("{} / {} = {}", v[0], v[1], result);
    }
    Ok(())
}
