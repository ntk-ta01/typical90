use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        q: usize,
        query: [i64; q],
    }
    a.sort_unstable();

    for b in query {
        let i = binary_search(&a, b);
        let x = if 0 < i {
            (a[i - 1] - b).abs().min((a[i] - b).abs())
        } else {
            (a[i] - b).abs()
        };
        println!("{}", x);
    }
}

fn binary_search(a: &[i64], b: i64) -> usize {
    let mut ng = 1_usize.wrapping_neg();
    let mut ok = a.len() - 1;
    if a[ok] < b {
        return ok;
    }
    // a[ok] は常にb 以上
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        if b <= a[mid] {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
