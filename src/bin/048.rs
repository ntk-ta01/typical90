use std::collections::BinaryHeap;

use proconio::input;
fn main() {
    input! {
        n: usize,
        k: usize,
        data: [(usize, usize); n],
    }
    let mut ps = BinaryHeap::new();
    let mut used = vec![false; n];
    for (i, &(_, bi)) in data.iter().enumerate() {
        ps.push((bi, i));
    }

    let mut ans = 0;
    for _ in 0..k {
        let (p, i) = ps.pop().unwrap();
        ans += p;
        if !used[i] {
            used[i] = true;
            ps.push((data[i].0 - p, i))
        }
    }
    println!("{}", ans);
}
