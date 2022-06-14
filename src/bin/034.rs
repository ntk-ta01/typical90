use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut map = HashMap::new();
    let mut idx = 0;
    let mut ans = 0;
    for i in 0..n {
        // idxを進める
        while idx < n && (map.len() < k || map.contains_key(&a[idx])) {
            *map.entry(a[idx]).or_insert(0) += 1;
            idx += 1;
        }
        ans = ans.max(idx - i);

        // a[i]を削除
        *map.get_mut(&a[i]).unwrap() -= 1;
        if map[&a[i]] == 0 {
            map.remove(&a[i]).unwrap();
        }
    }
    println!("{}", ans);
}
