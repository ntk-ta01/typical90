use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut dp = vec![vec![0; 26]; n + 1];
    const MOD: i64 = 1_000_000_007;
    for i in 0..n {
        for j in 0..26 {
            dp[i + 1][j] = dp[i][j];
            if s[i] as usize - 'a' as usize == j {
                if s[i] == 'a' {
                    dp[i + 1][s[i] as usize - 'a' as usize] += 1;
                } else if s[i] == 't' {
                    dp[i + 1][s[i] as usize - 'a' as usize] += dp[i][0];
                } else if s[i] == 'c' {
                    dp[i + 1][s[i] as usize - 'a' as usize] += dp[i]['t' as usize - 'a' as usize];
                } else if s[i] == 'o' {
                    dp[i + 1][s[i] as usize - 'a' as usize] += dp[i]['c' as usize - 'a' as usize];
                } else if s[i] == 'd' {
                    dp[i + 1][s[i] as usize - 'a' as usize] += dp[i]['o' as usize - 'a' as usize];
                } else if s[i] == 'e' {
                    dp[i + 1][s[i] as usize - 'a' as usize] += dp[i]['d' as usize - 'a' as usize];
                } else if s[i] == 'r' {
                    dp[i + 1][s[i] as usize - 'a' as usize] += dp[i]['e' as usize - 'a' as usize];
                }
                dp[i + 1][s[i] as usize - 'a' as usize] %= MOD;
            }
        }
    }
    println!("{}", dp[n]['r' as usize - 'a' as usize])
}
