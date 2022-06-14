use proconio::input;
fn main() {
    input! {
        k: u64,
    }
    let ds = enum_divisors(k);
    let mut ans = 0;
    for &a in ds.iter() {
        for &b in ds.iter() {
            if k / a < b {
                continue;
            }
            if k % (a * b) != 0 {
                continue;
            }
            let c = k / (a * b);
            if a <= b && b <= c {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

fn enum_divisors(k: u64) -> Vec<u64> {
    let mut res = vec![];
    for a in 1..=k {
        if a * a > k {
            break;
        }
        if k % a == 0 {
            res.push(a);
            if k / a != a {
                res.push(k / a);
            }
        }
    }
    res.sort_unstable();
    res
}

#[allow(unused)]
fn prime_factorize(mut k: i64) -> Vec<(i64, i32)> {
    let n = k;
    let mut res = vec![];
    for a in 2..n {
        if a * a > n {
            break;
        }
        if k % a != 0 {
            continue;
        }
        let mut ex = 0;

        while k % a == 0 {
            ex += 1;
            k /= a;
        }

        res.push((a, ex));
    }
    if k != 1 {
        res.push((k, 1));
    }
    res
}
