use itertools::Itertools;
use proconio::input;
use std::collections::VecDeque;
fn main() {
    input! {
        q: usize,
        query: [(usize, usize); q],
    }
    let mut ans = vec![];
    let mut deque = VecDeque::new();
    for (t, x) in query {
        match t {
            1 => deque.push_front(x),
            2 => deque.push_back(x),
            3 => ans.push(deque[x - 1]),
            _ => unreachable!(),
        }
    }
    println!("{}", ans.iter().join("\n"));
}
