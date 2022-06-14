use proconio::input;
const MOD: usize = 1_000_000_007;
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut ans = 0;
    if n == 1 {
        ans = k;
    } else if k != 1 {
        ans += k * (k - 1);
        ans %= MOD;
        ans *= pow(k - 2, n - 2);
        ans %= MOD;
    }
    println!("{}", ans);
}

fn pow(x: usize, n: usize) -> usize {
    let mut ret = 1;
    let mut x = x;
    let mut n = n;
    while n > 0 {
        if n % 2 == 1 {
            ret = ret * x % MOD;
        }
        x = x * x % MOD;
        n >>= 1;
    }
    ret % MOD
}
