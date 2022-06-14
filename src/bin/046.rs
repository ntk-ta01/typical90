use proconio::input;
use std::collections::BTreeMap;
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }
    // mod 46 が0になるかチェック
    const MOD: usize = 46;
    let mut mpa = BTreeMap::new();
    let mut mpb = BTreeMap::new();
    let mut mpc = BTreeMap::new();
    for i in a {
        *mpa.entry(i % MOD).or_insert(0) += 1;
    }
    for i in b {
        *mpb.entry(i % MOD).or_insert(0) += 1;
    }
    for i in c {
        *mpc.entry(i % MOD).or_insert(0) += 1;
    }
    let mut ans = 0_usize;
    for (&ka, &va) in mpa.iter() {
        for (&kb, &vb) in mpb.iter() {
            for (&kc, &vc) in mpc.iter() {
                if (ka + kb + kc) % MOD == 0 {
                    ans += va * vb * vc;
                }
            }
        }
    }
    println!("{}", ans);
}
