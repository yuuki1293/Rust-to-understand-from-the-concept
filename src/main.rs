enum Sign {
    Positive,
    Zero,
    Negative,
}

fn determine_sign(x: i32) -> Sign {
    if x > 0 {
        Sign::Positive
    } else if x < 0 {
        Sign::Negative
    } else {
        Sign::Zero
    }
}
