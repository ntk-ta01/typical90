// 解説AC くやしい あの WA が出たのですが
use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }
    // next[i][c] := i文字目以降で初めて文字cが出てくるindex
    let next = {
        let mut res = vec![vec![n; 26]; n + 1];
        for i in (0..n).rev() {
            for j in 0..26 {
                res[i][j] = res[i + 1][j];
            }
            res[i][s[i] as usize - 'a' as usize] = i;
        }
        res
    };
    let mut ans = vec![];
    let mut idx = 0;
    for i in 0..k {
        // ansのi文字目を選ぶ
        // next[idx][c]を選ぶには、next[idx][c]番目以降の文字が、k-i文字以上ある
        // n - next[idx][c] >= k - i
        for c in 0..26 {
            if n - next[idx][c] >= k - i {
                ans.push((b'a' + c as u8) as char);
                idx = next[idx][c] + 1;
                break;
            }
        }
    }
    println!("{}", ans.iter().collect::<String>());
}
