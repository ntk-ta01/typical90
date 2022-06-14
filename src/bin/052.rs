use proconio::input;
const MOD: usize = 1_000_000_007;
fn main() {
    input! {
        n: usize,
        a: [[usize; 6]; n],
    }
    let mut dp = vec![vec![0; 6]; n];
    for j in 0..6 {
        dp[0][j] = a[0][j];
    }
    for i in 1..n {
        for j in 0..6 {
            for k in 0..6 {
                dp[i][j] += dp[i - 1][k] * a[i][j];
                dp[i][j] %= MOD;
            }
        }
    }
    println!("{}", dp[n - 1].iter().fold(0_usize, |x, &y| (x + y) % MOD));
}
