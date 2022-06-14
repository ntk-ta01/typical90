use num_integer::gcd;
use proconio::input;
fn main() {
    input! {
        a: u128,
        b: u128,
    }
    const INF: u128 = 1_000_000_000_000_000_000;
    let ans = a * b / gcd(a, b);
    if INF < ans {
        println!("Large");
    } else {
        println!("{}", ans);
    }
}
