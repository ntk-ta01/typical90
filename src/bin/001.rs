use proconio::input;
fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        a: [usize; n],
    }
    // 長さの最小値をx以上にできるか二分探索
    let is_ok = |mid: usize| -> bool {
        let mut len;
        let mut s = 0;
        let mut pre = 0;
        for &ai in a.iter() {
            len = ai - pre;
            // 長さがmid以上なら切る
            if s < k && mid <= len {
                s += 1;
                pre = ai;
            }
        }
        len = l - pre;
        mid <= len && k <= s
    };
    let mut ng = l;
    let mut ok = 1;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
