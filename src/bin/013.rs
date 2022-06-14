use proconio::{input, marker::Usize1};
use std::{cmp::Reverse, collections::BinaryHeap};
fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1, usize); m],
    }
    let mut g = vec![vec![]; n];
    for &(a, b, c) in edges.iter() {
        g[a].push((b, c));
        g[b].push((a, c));
    }
    let dijkstra = |s: usize| -> Vec<usize> {
        const INF: usize = usize::max_value();
        let mut dist = vec![INF; n];
        let mut heap = BinaryHeap::new();
        dist[s] = 0;
        heap.push((Reverse(0), s));
        while !heap.is_empty() {
            let (Reverse(cost), now) = heap.pop().unwrap();
            if dist[now] < cost {
                continue;
            }
            for &(next, next_c) in g[now].iter() {
                if dist[next] > dist[now] + next_c {
                    dist[next] = dist[now] + next_c;
                    heap.push((Reverse(dist[next]), next));
                }
            }
        }
        dist
    };
    let dist1 = dijkstra(0);
    let distn = dijkstra(n - 1);
    for (&d1, &dn) in dist1.iter().zip(distn.iter()) {
        println!("{}", d1 + dn);
    }
}
