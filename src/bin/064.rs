use proconio::{input, marker::Usize1};
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        query: [(Usize1, Usize1, i64); q],
    }
    let mut ans = 0;
    let mut b = vec![];
    for i in 1..n {
        b.push(a[i] - a[i - 1]);
        ans += b[i - 1].abs();
    }
    b.push(0);
    for (l, r, v) in query {
        ans -= if 0 < l { b[l - 1].abs() } else { 0 } + b[r].abs();
        if 0 < l {
            b[l - 1] += v;
        }
        if r < n - 1 {
            b[r] -= v;
        }
        ans += if 0 < l { b[l - 1].abs() } else { 0 } + b[r].abs();
        println!("{}", ans);
    }
}
