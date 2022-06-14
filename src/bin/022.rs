use num_integer::gcd;
use proconio::input;
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let g = gcd(gcd(a, b), c);
    let ans = a / g - 1 + b / g - 1 + c / g - 1;
    println!("{}", ans);
}
