use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        n: usize,
        k: usize,
        p: usize,
        mut a: [usize; n],
    }
    let mut b = a.clone();
    a = a.into_iter().take(n / 2).collect_vec();
    b = b.into_iter().skip(n / 2).collect_vec();
    let mut a_sel = vec![vec![]; n + 1];
    for bit in 0..(1 << (n / 2)) {
        let mut count = 0;
        let mut sum = 0;
        for i in 0..(n / 2) {
            if (bit >> i) & 1 == 1 {
                count += 1;
                sum += a[i];
            }
        }
        if count <= k {
            a_sel[count].push(sum);
        }
    }
    let mut b_sel = vec![vec![]; n + 1];
    for bit in 0..(1 << (n - (n / 2))) {
        let mut count = 0;
        let mut sum = 0;
        for i in 0..(n - (n / 2)) {
            if (bit >> i) & 1 == 1 {
                count += 1;
                sum += b[i]
            }
        }
        if count <= k {
            b_sel[count].push(sum);
        }
    }
    for i in 0..k + 1 {
        b_sel[i].sort_unstable();
    }
    let mut ans = 0;
    for (i, row) in a_sel.iter().enumerate().take(k + 1) {
        for sum_i in row {
            // i個選んで、sum_i円である
            // k - i個選んでp - sum_i円以下である選び方の和が答え
            if *sum_i <= p {
                let now = binary_search(&b_sel[k - i], p - *sum_i);
                ans += now;
            }
        }
    }
    println!("{}", ans);
}

fn binary_search(seq: &[usize], a: usize) -> usize {
    let mut ok = 1_usize.wrapping_neg();
    let mut ng = seq.len();
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if seq[mid] <= a {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok + 1
}
