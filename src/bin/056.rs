use proconio::input;
fn main() {
    input! {
        n: usize,
        s: usize,
        data: [[usize; 2]; n],
    }
    let mut dp = vec![vec![0; s + 1]; n + 1];
    dp[0][0] = 3;
    for i in 0..n {
        for j in 0..s {
            if dp[i][j] == 0 {
                continue;
            }
            for k in 0..2 {
                let sum = j + data[i][k];
                if sum <= s {
                    // i+1日目にk番目の福袋を選ぶ
                    dp[i + 1][sum] = k + 1;
                }
            }
        }
    }
    // 復元
    let mut ans = vec![];
    let mut i = n;
    let mut q = s;
    while q > 0 {
        match dp[i][q] {
            0 => {
                println!("Impossible");
                return;
            }
            1 => {
                ans.push(1);
                q -= data[i - 1][0];
            }
            2 => {
                ans.push(2);
                q -= data[i - 1][1];
            }
            _ => unreachable!(),
        }
        i -= 1;
    }
    println!(
        "{}",
        ans.iter()
            .rev()
            .map(|k| if *k == 1 { 'A' } else { 'B' })
            .collect::<String>()
    );
}
