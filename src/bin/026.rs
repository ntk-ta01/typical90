use itertools::Itertools;
use proconio::{input, marker::Usize1};
fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
    }
    let mut g = vec![vec![]; n];
    for (a, b) in edges {
        g[a].push(b);
        g[b].push(a);
    }
    let mut white = vec![true; n];
    let mut visited = vec![false; n];
    dfs(0, &g, &mut visited, &mut white);
    let white_count = white.iter().filter(|w| **w).count();
    let ans = if n / 2 <= white_count {
        white
            .iter()
            .enumerate()
            .filter(|(_, w)| **w)
            .take(n / 2)
            .map(|(i, _)| i + 1)
            .join(" ")
    } else {
        white
            .iter()
            .enumerate()
            .filter(|(_, w)| !**w)
            .take(n / 2)
            .map(|(i, _)| i + 1)
            .join(" ")
    };
    println!("{}", ans);
}

fn dfs(v: usize, g: &[Vec<usize>], visited: &mut [bool], white: &mut [bool]) {
    for nv in g[v].iter() {
        if visited[*nv] {
            continue;
        }
        visited[*nv] = true;
        white[*nv] = !white[v];
        dfs(*nv, g, visited, white);
    }
}
