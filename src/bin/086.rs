use proconio::{input, marker::Usize1};
fn main() {
    input! {
        n: usize,
        q: usize,
        query: [(Usize1, Usize1, Usize1, usize); q],
    }
    let mut ans = 0;
    const MOD: i64 = 1_000_000_007;
    for i in 0..60 {
        let mut now = 0;
        // 未確定が0か1かで全探索
        for bit in 0..(1 << n) {
            let mut bs = vec![0; n];
            for (j, b) in bs.iter_mut().enumerate() {
                if (bit >> j) & 1 == 1 {
                    *b = 1;
                }
            }
            // 条件を満たしているかチェック
            if query
                .iter()
                .all(|&(x, y, z, w)| (w >> i) & 1 == (bs[x] | bs[y] | bs[z]))
            {
                now += 1;
            }
        }

        if ans == 0 {
            ans += now;
        } else {
            ans *= now;
            ans %= MOD;
        }
    }
    println!("{}", ans);
}
