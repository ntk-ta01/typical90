use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        query: [(usize, usize, usize); q],
    }
    let mut rot = 0;
    let mut ans = vec![];
    for (t, x, y) in query {
        match t {
            1 => {
                a.swap((x - 1 + rot) % n, (y - 1 + rot) % n);
            }
            2 => {
                rot += n - 1;
                rot %= n;
            }
            3 => ans.push(a[(x - 1 + rot) % n]),
            _ => unreachable!(),
        }
    }
    println!("{}", ans.iter().join("\n"));
}
