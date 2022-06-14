use proconio::{fastout, input, marker::Usize1};
#[fastout]
fn main() {
    input! {
        n: usize,
        data: [(usize, usize); n],
        q: usize,
    }
    let mut acc1 = vec![0; n + 1];
    let mut acc2 = vec![0; n + 1];
    for (i, (c, p)) in data.into_iter().enumerate() {
        if c == 1 {
            acc1[i + 1] += p;
        } else {
            acc2[i + 1] += p;
        }
    }
    for i in 0..n {
        acc1[i + 1] += acc1[i];
        acc2[i + 1] += acc2[i];
    }
    for _ in 0..q {
        input! {
            l: Usize1,
            r: Usize1,
        }
        println!("{} {}", acc1[r + 1] - acc1[l], acc2[r + 1] - acc2[l]);
    }
}
