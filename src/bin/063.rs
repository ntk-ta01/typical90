use proconio::{input, marker::Usize1};
fn main() {
    input! {
        h: usize,
        w: usize,
        ps: [[Usize1; w]; h],
    }
    let mut ans = vec![0; h * w];
    for bit in 0..1 << h {
        let mut rec = vec![0; h * w];
        for j in 0..w {
            let mut s = None;
            let mut good = true;
            let mut count = 0;
            for (i, p) in ps.iter().enumerate() {
                if (bit >> i) & 1 == 1 {
                    count += 1;
                    match s {
                        None => s = Some(p[j]),
                        Some(v) => {
                            if v != p[j] {
                                good = false;
                            }
                        }
                    }
                }
            }
            if !good {
                continue;
            }
            if let Some(v) = s {
                rec[v] += count;
            }
        }
        for v in 0..h * w {
            ans[v] = ans[v].max(rec[v]);
        }
    }
    // eprintln!("{:?}", ans);
    println!("{}", ans.iter().max().unwrap());
}
