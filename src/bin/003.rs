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
    let d = tree_diameter(&g);
    println!("{}", d + 1);
}

fn tree_diameter(g: &[Vec<usize>]) -> usize {
    let n = g.len();
    let mut dist = vec![n; n];
    dist[0] = 0;
    let mut m = 0;
    let mut mi = 0;
    dfs(0, g, &mut dist, &mut m, &mut mi);
    let mut dist = vec![n; n];
    dist[mi] = 0;
    let mut m = 0;
    dfs(mi, g, &mut dist, &mut m, &mut mi);
    m
}

fn dfs(v: usize, g: &[Vec<usize>], dist: &mut [usize], m: &mut usize, mi: &mut usize) {
    for &nv in g[v].iter() {
        if dist[nv] != dist.len() {
            continue;
        }
        dist[nv] = dist[v] + 1;
        if *m < dist[nv] {
            *m = dist[nv];
            *mi = nv;
        }
        dfs(nv, g, dist, m, mi);
    }
}
