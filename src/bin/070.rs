use proconio::input;
fn main() {
    input! {
        n: usize,
        mut pos: [(i64, i64); n],
    }
    pos.sort_by_key(|p| p.0);
    let ans_x = if n % 2 == 1 {
        pos[n / 2].0
    } else {
        (pos[n / 2].0 + pos[n / 2 - 1].0) / 2
    };
    pos.sort_by_key(|p| p.1);
    let ans_y = if n % 2 == 1 {
        pos[n / 2].1
    } else {
        (pos[n / 2].1 + pos[n / 2 - 1].1) / 2
    };
    let mut ans = 0;
    for (x, y) in pos {
        ans += (ans_x - x).abs() + (ans_y - y).abs();
    }
    println!("{}", ans);
}
