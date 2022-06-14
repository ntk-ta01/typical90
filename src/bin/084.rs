use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut a = vec![];
    let mut ch = s[0];
    let mut len = 0;
    for c in s {
        if c == ch {
            len += 1;
        } else {
            ch = c;
            a.push(len);
            len = 1;
        }
    }
    a.push(len);
    let mut ans = 0;
    for v in a {
        ans += v * (n - v);
    }
    println!("{}", ans / 2);
}
