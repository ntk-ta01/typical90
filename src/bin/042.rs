use proconio::input;
const MOD: usize = 1_000_000_007;
fn main() {
    input! {
        k: usize,
    }
    if k % 9 != 0 {
        println!("{}", 0);
    } else {
        // dp[i] := 桁和がiとなる正整数の数 kが9の倍数であれば、これが答え
        // dp[1] = 1
        // dp[2] = 2
        // dpの定義を、桁和がiとなる9の倍数の数としそうになったが、
        // そうするとiが9の倍数でないときはすべて0になってしまう
        let mut dp = vec![0; k + 1];
        dp[1] = 1;
        dp[2] = 1;
        dp[3] = 1;
        dp[4] = 1;
        dp[5] = 1;
        dp[6] = 1;
        dp[7] = 1;
        dp[8] = 1;
        dp[9] = 1;
        for i in 1..=k {
            for j in 1..10 {
                if i < j {
                    break;
                }
                dp[i] += dp[i - j];
                dp[i] %= MOD;
            }
        }
        println!("{}", dp[k]);
    }
}
