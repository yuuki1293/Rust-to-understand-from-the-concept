const N_MAX: usize = 100000000;
const N_THREAD: usize = 4;

const N_ELEM_PER_THREAD: usize = N_MAX / N_THREAD;
const RESIDUAL: usize = N_MAX % N_THREAD;

fn main() -> std::thread::Result<()> {
    if RESIDUAL != 0 {
        panic!("invalid combination of N_MAX and N_THREAD");
    }

    let mut thrd = Vec::new();
    let v = (1..=N_MAX).collect::<Vec<usize>>();

    // 1..=N_MAXをN_THREADに分割して、それぞれの和をスレッドで計算
    for ii in 0..N_THREAD {
        let ist = ii * N_ELEM_PER_THREAD;
        let ien = ist + N_ELEM_PER_THREAD;
        let vv = (&v[ist..ien]).to_owned();
        let th = std::thread::spawn(move || vv.into_iter().sum::<usize>());
        thrd.push(th);
    }

    // 各スレッドで計算した値を集めて、その和を取り、全体の和を求める
    let ans: usize = thrd.into_iter().map(|r| r.join().unwrap()).sum::<usize>();
    println!("{}", ans);
    assert_eq!(ans, N_MAX * (N_MAX + 1) / 2);
    Ok(())
}
