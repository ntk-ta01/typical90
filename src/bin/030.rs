use proconio::input;
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut p = vec![0; n + 1];
    for i in 2..=n {
        if p[i] > 0 {
            continue;
        }
        for j in (1..).into_iter().take_while(|&j| i * j <= n) {
            p[i * j] += 1;
        }
    }
    let ans = p.iter().filter(|&&i| k <= i).count();
    println!("{}", ans);
}
