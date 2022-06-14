use proconio::{input, marker::Usize1};
fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    }
    let mut count = vec![0; n];
    for (a, b) in edges {
        if a > b {
            count[a] += 1;
        } else {
            count[b] += 1;
        }
    }
    println!("{}", count.iter().filter(|c| **c == 1).count());
}
