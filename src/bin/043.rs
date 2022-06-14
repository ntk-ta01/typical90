// グラフを拡張するのが見えなかったので解説を見た 実装は一瞬
// ↑ WAが出たので一瞬ではありませんでした え あんまり納得していないな
use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::VecDeque;

const DIJ: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];

fn main() {
    input! {
        h: usize,
        w: usize,
        rs: Usize1,
        cs: Usize1,
        rt: Usize1,
        ct: Usize1,
        s: [Chars; h],
    }
    const INF: usize = usize::max_value();
    let mut que1 = VecDeque::new();
    let mut que2 = VecDeque::new();
    let mut dist = vec![vec![vec![INF; 4]; w]; h];
    for i in 0..4 {
        dist[rs][cs][i] = 0;
    }
    que1.push_back((0, rs, cs, 4));
    while !que1.is_empty() || !que2.is_empty() {
        let (cost, r, c, pre) = if !que1.is_empty() {
            que1.pop_front().unwrap()
        } else {
            que2.pop_front().unwrap()
        };
        if pre != 4 && dist[r][c][pre] < cost {
            continue;
        }
        for (i, &(di, dj)) in DIJ.iter().enumerate() {
            let nr = r + di;
            let nc = c + dj;
            if h <= nr || w <= nc {
                continue;
            }
            if s[nr][nc] == '#' {
                continue;
            }
            if pre == 4 || pre == i {
                if dist[nr][nc][i] > dist[r][c][i] {
                    dist[nr][nc][i] = dist[r][c][i];
                    que1.push_back((dist[nr][nc][i], nr, nc, i));
                }
            } else if dist[nr][nc][i] > dist[r][c][pre] + 1 {
                dist[nr][nc][i] = dist[r][c][pre] + 1;
                que2.push_back((dist[nr][nc][i], nr, nc, i));
            }
        }
    }
    println!("{}", dist[rt][ct].iter().min().unwrap());
}
