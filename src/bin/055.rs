use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        a: [usize; n],
    }
    let mut ans = 0;
    for i1 in 0..n {
        for i2 in i1 + 1..n {
            for i3 in i2 + 1..n {
                for i4 in i3 + 1..n {
                    for i5 in i4 + 1..n {
                        if a[i1] * a[i2] % p * a[i3] % p * a[i4] % p * a[i5] % p == q {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
