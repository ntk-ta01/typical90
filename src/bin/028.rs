use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        n: usize,
        query: [(usize, usize, usize, usize); n],
    }
    const N: usize = 1000;
    let mut p = vec![vec![0; N + 1]; N + 1];
    for (lx, ly, rx, ry) in query {
        p[ly][lx] += 1;
        p[ly][rx] -= 1;
        p[ry][lx] -= 1;
        p[ry][rx] += 1;
    }
    for i in 0..N + 1 {
        for j in 0..N {
            p[i][j + 1] += p[i][j];
        }
    }
    for i in 0..N {
        for j in 0..N + 1 {
            p[i + 1][j] += p[i][j];
        }
    }
    let mut count = vec![0; n + 1];
    for i in 0..N {
        for j in 0..N {
            count[p[i][j] as usize] += 1;
        }
    }
    println!("{}", count.iter().skip(1).join("\n"));
}
