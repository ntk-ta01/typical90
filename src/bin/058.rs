use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut zs = vec![];
    let mut s = HashSet::new();
    let mut x = n;
    while !s.contains(&x) {
        zs.push(x);
        s.insert(x);
        let y = {
            let mut y = 0;
            let mut now = x;
            while now > 0 {
                y += now % 10;
                now /= 10;
            }
            y
        };
        x = (x + y) % 100000;
    }
    let dup_index = zs.iter().position(|z| *z == x).unwrap();
    if k < zs.len() {
        println!("{}", zs[k]);
    } else {
        let i = (k - dup_index) % (zs.len() - dup_index);
        println!("{}", zs[dup_index + i]);
    }
}
