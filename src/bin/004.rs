use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i64; w]; h],
    }
    let mut row_sum = vec![0; h];
    let mut col_sum = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            row_sum[i] += a[i][j];
            col_sum[j] += a[i][j];
        }
    }
    for i in 0..h {
        for j in 0..w {
            if j != w - 1 {
                print!("{} ", row_sum[i] + col_sum[j] - a[i][j]);
            } else {
                println!("{}", row_sum[i] + col_sum[j] - a[i][j]);
            }
        }
    }
}
