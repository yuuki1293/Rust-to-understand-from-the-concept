async fn sum_func(n: usize) -> usize {
    let ans = (1..=n).into_iter().sum::<usize>();
    println!("{}", ans);
    ans
}

fn main() {
    sum_func(10000000);
    println!("called")
}
