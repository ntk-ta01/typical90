use proconio::input;
fn main() {
    input! {
        a: usize,
        b: u32,
        c: usize,
    }
    let v1 = a;
    let v2 = c.pow(b);
    if v1 < v2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
