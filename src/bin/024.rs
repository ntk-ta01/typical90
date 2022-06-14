use proconio::input;
fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }
    let diff = a
        .iter()
        .zip(b.iter())
        .map(|(&x, &y)| (x - y).abs())
        .sum::<i64>();
    if diff <= k && (k - diff) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
