use proconio::input;
fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[i64; w]; h],
        b: [[i64; w]; h],
    }
    let mut count = 0;
    for i in 0..h - 1 {
        for j in 0..w - 1 {
            let diff = b[i][j] - a[i][j];
            if diff != 0 {
                count += diff.abs();
            }
            a[i][j] += diff;
            a[i][j + 1] += diff;
            a[i + 1][j] += diff;
            a[i + 1][j + 1] += diff;
        }
    }
    if a == b {
        println!("Yes");
        println!("{}", count);
    } else {
        println!("No");
    }
}
