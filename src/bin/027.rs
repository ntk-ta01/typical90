use proconio::{fastout, input};
use std::collections::HashSet;
#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut set = HashSet::new();
    for (i, st) in s.into_iter().enumerate() {
        if !set.contains(&st) {
            println!("{}", i + 1);
            set.insert(st);
        }
    }
}
