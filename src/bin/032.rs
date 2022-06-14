use permutohedron::LexicalPermutation;
use proconio::{input, marker::Usize1};
fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        m: usize,
        edges: [(Usize1, Usize1); m],
    }
    let mut bad_rel = vec![vec![false; n]; n];
    for (a, b) in edges {
        bad_rel[a][b] = true;
        bad_rel[b][a] = true;
    }
    let mut ord = (0..n).collect::<Vec<_>>();
    const INF: usize = usize::max_value();
    let mut ans = INF;
    loop {
        let mut now = 0;
        for (j, &i) in ord.iter().enumerate() {
            now += a[i][j];
        }
        if ord
            .iter()
            .zip(ord.iter().skip(1))
            .all(|(x, y)| !bad_rel[*x][*y])
        {
            ans = ans.min(now);
        }
        if !ord.next_permutation() {
            break;
        }
    }
    if ans != INF {
        println!("{}", ans);
    } else {
        println!("{}", -1);
    }
}
