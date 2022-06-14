use proconio::input;
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        mut b: [i64; n],
    }
    a.sort_unstable();
    b.sort_unstable();
    let ans: i64 = a.iter().zip(b.iter()).map(|(&x, &y)| (x - y).abs()).sum();
    println!("{}", ans);
}
