use proconio::{input, marker::Chars};

const DIJ: [(usize, usize); 4] = [(1, 0), (0, 1), (!0, 0), (0, !0)];
fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let mut answer = 0;
    let mut visited = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            let mut depth = 0;
            dfs(
                (i, j),
                DIJ.len(),
                &c,
                (i, j),
                &mut visited,
                &mut depth,
                &mut answer,
            );
        }
    }

    println!("{}", if answer != 0 { answer } else { -1 });
}

fn dfs(
    v: (usize, usize),
    prev: usize,
    g: &[Vec<char>],
    s: (usize, usize),
    visited: &mut Vec<Vec<bool>>,
    depth: &mut i32,
    answer: &mut i32,
) {
    if *depth > 0 && v == s {
        if *answer < *depth {
            *answer = *depth;
        }
        return;
    }
    for (dir, &(di, dj)) in DIJ.iter().enumerate() {
        let ni = v.0 + di;
        let nj = v.1 + dj;
        if prev < DIJ.len() && dir == (prev + 2) % DIJ.len() {
            continue;
        }
        if ni >= visited.len() || nj >= visited[v.0].len() || visited[ni][nj] {
            continue;
        }
        if g[ni][nj] == '#' {
            continue;
        }
        visited[ni][nj] = true;
        *depth += 1;
        dfs((ni, nj), dir, g, s, visited, depth, answer);
        visited[ni][nj] = false;
        *depth -= 1;
    }
}
